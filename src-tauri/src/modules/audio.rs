use crate::get_event_channel;
use crate::modules::events::RecordEvent;
use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Sample, SupportedStreamConfig,
};
use hound::{WavSpec, WavWriter};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{
    cell::RefCell,
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::BufWriter,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    time::{Duration, Instant},
};

const RECORDING_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../recorded.wav");
const MAX_RECORDING_DURATION_SECS: u64 = 60 * 5;
static RECORDING_ACTIVE: AtomicBool = AtomicBool::new(false);

type WavWriterHandle = Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>;

thread_local! {
    static CURRENT_STREAM: RefCell<Option<cpal::Stream>> = RefCell::new(None);
}

lazy_static! {
    static ref CURRENT_WRITER: Mutex<Option<WavWriterHandle>> = Mutex::new(None);
    // static ref APP_HANDLE: Mutex<Option<AppHandle>> = Mutex::new(None);
    static ref LAST_SEND_TIME: Mutex<Instant> = Mutex::new(Instant::now());
    static ref SAMPLE_BUFFER: Mutex<Vec<i16>> = Mutex::new(Vec::new());
}

const THROTTLE_DURATION: Duration = Duration::from_millis(10); // 100 -> 10 раз в секунду

#[derive(Debug, Serialize, Deserialize)]
struct AudioDevice {
    id: String,
    name: String,
}

/// Генерирует хеш для имени устройства
fn get_device_hash(device_name: &str) -> String {
    let mut hasher = DefaultHasher::new();
    device_name.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Создает спецификацию WAV файла на основе конфигурации потока
fn create_wav_spec(config: &SupportedStreamConfig) -> WavSpec {
    WavSpec {
        channels: 1,
        sample_rate: config.sample_rate().0 as _,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    }
}
// Function to get the current time in milliseconds since the Unix Epoch
fn get_current_time_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Записывает входные данные в WAV файл и возвращает пиковое значение
fn write_input_data<T>(input: &[T], writer: &WavWriterHandle) -> i16
where
    T: Sample,
    i16: Sample + FromSample<T>,
{
    let mut current_peak: i16 = 0;
    let mut samples: Vec<i16> = Vec::with_capacity(input.len());

    // Конвертируем входные данные в i16 и находим пиковое значение
    for &sample in input.iter() {
        let sample = i16::from_sample(sample);
        samples.push(sample);

        current_peak = if sample > 0 {
            current_peak.max(sample.min(i16::MAX))
        } else {
            current_peak.min(sample.max(i16::MIN))
        };
    }

    // Запускаем запись в файл в отдельном потоке
    let writer_clone = writer.clone();
    let samples_clone = samples.clone();
    std::thread::spawn(move || {
        if let Ok(mut guard) = writer_clone.lock() {
            if let Some(writer) = guard.as_mut() {
                for &sample in samples_clone.iter() {
                    writer.write_sample(sample).ok();
                }
            }
        }
    });

    current_peak
}

/// Обработчик аудио данных - записывает данные и отправляет события о прогрессе
fn handle_audio_data<T>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    i16: Sample + FromSample<T>,
{
    // Записываем данные и получаем пиковое значение
    let peak = write_input_data(input, writer);

    // Отправляем событие о прогрессе, если нужно
    if let Some(channel) = get_event_channel() {
        let mut last_send = LAST_SEND_TIME.lock().unwrap();
        if last_send.elapsed() >= THROTTLE_DURATION {
            channel
                .send(RecordEvent::Progress {
                    timestamp: get_current_time_millis(),
                    peak,
                })
                .ok();

            *last_send = Instant::now();
        }
    }
}

/// Получает список доступных микрофонов
pub fn get_microphones() -> Result<String> {
    let host = cpal::default_host();
    let devices = host.input_devices()?;

    let devices_list: Vec<AudioDevice> = devices
        .filter_map(|device| {
            device.name().ok().map(|name| AudioDevice {
                id: get_device_hash(&name),
                name: name.clone(),
            })
        })
        .collect();

    Ok(serde_json::to_string(&devices_list)?)
}

/// Получает устройство ввода по его идентификатору
fn get_input_device(device_id: &str) -> Result<Device> {
    let host = cpal::default_host();
    let device = if !device_id.is_empty() {
        host.input_devices()?.find(|device| {
            device
                .name()
                .map(|name| get_device_hash(&name) == device_id)
                .unwrap_or(false)
        })
    } else {
        host.default_input_device()
    }
    .ok_or_else(|| anyhow::anyhow!("Не удалось найти устройство ввода"))?;

    Ok(device)
}

/// Обрабатывает ошибку аудиопотока и безопасно останавливает запись
fn handle_stream_error(err: cpal::StreamError) {
    eprintln!("Ошибка потока: {}", err);
    let _ = stop();
    println!("Запись остановлена из-за ошибки потока");
}

/// Записывает аудио с выбранного устройства
pub fn record(device_id: &str) -> Result<()> {
    if RECORDING_ACTIVE.load(Ordering::SeqCst) {
        return Err(anyhow::anyhow!("Запись уже идет"));
    }

    let device = get_input_device(device_id)?;
    println!("Устройство ввода: {}", device.name()?);

    let config = device
        .default_input_config()
        .map_err(|e| anyhow::anyhow!("Ошибка получения конфигурации: {}", e))?;

    let spec = create_wav_spec(&config);
    let writer = WavWriter::create(RECORDING_PATH, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));
    let writer_clone = writer.clone();

    // Отправка события
    if let Some(channel) = get_event_channel() {
        channel
            .send(RecordEvent::Start {
                timestamp: get_current_time_millis(),
            })
            .ok();
    }

    println!("Начало записи в файл: {}", RECORDING_PATH);

    let err_fn = move |err| handle_stream_error(err);

    // Устанавливаем флаг активной записи
    RECORDING_ACTIVE.store(true, Ordering::SeqCst);

    // Создаем поток записи
    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    handle_audio_data::<i8>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    handle_audio_data::<i16>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    handle_audio_data::<i32>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    handle_audio_data::<f32>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )?,
        format => return Err(anyhow::anyhow!("Неподдерживаемый формат: {format}")),
    };

    // Сохраняем текущий поток и writer для возможности остановки
    CURRENT_STREAM.with(|s| s.borrow_mut().replace(stream));
    CURRENT_WRITER.lock().unwrap().replace(writer);

    // Запускаем поток записи
    CURRENT_STREAM.with(|s| {
        if let Some(stream) = s.borrow().as_ref() {
            stream.play()?;
        }
        Ok::<_, anyhow::Error>(())
    })?;

    // Запускаем таймер максимальной длительности записи в отдельном потоке
    std::thread::spawn(move || {
        let start_time = Instant::now();
        while RECORDING_ACTIVE.load(Ordering::SeqCst) {
            if start_time.elapsed() >= Duration::from_secs(MAX_RECORDING_DURATION_SECS) {
                println!(
                    "Достигнут максимальный лимит записи ({} секунд)",
                    MAX_RECORDING_DURATION_SECS
                );
                let _ = stop();
                break;
            }
            std::thread::sleep(Duration::from_secs(1));
        }
    });

    Ok(())
}

/// Безопасно останавливает текущую запись
pub fn stop() -> Result<()> {
    if !RECORDING_ACTIVE.load(Ordering::SeqCst) {
        return Ok(());
    }
    // Устанавливаем флаг активной записи
    RECORDING_ACTIVE.store(false, Ordering::SeqCst);

    // Безопасно останавливаем поток и освобождаем ресурсы
    if let Some(s) = CURRENT_STREAM.with(|s| s.borrow_mut().take()) {
        drop(s);
    }

    // Окончательное завершение записи в файл, освобождение ресурсов и закрытие записи
    if let Ok(mut writer) = CURRENT_WRITER.lock() {
        if let Some(w) = writer.take() {
            if let Some(w) = w.lock().unwrap().take() {
                w.finalize()?;
            }
        }
    }

    if let Some(channel) = get_event_channel() {
        channel
            .send(RecordEvent::Stop {
                timestamp: get_current_time_millis(),
            })
            .ok();
    }
    println!("Запись остановлена");
    Ok(())
}
