use crate::get_event_channel_record;
use crate::modules::events::RecordEvent;
use crate::modules::transcribation::whisper_streamer::WhisperStreamer;
use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Sample, SizedSample, SupportedStreamConfig,
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
pub type SampleType = i8;
const BITS_PER_SAMPLE: u16 = 8;

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
    static ref SAMPLE_BUFFER: Mutex<Vec<SampleType>> = Mutex::new(Vec::new());
    static ref DEBUG_AUDIO: AtomicBool = AtomicBool::new(false);
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
        // bits_per_sample: 16,
        bits_per_sample: BITS_PER_SAMPLE,
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

/// Вычисляет пиковое значение из сэмплов и отправляет события о прогрессе
fn process_peaks(samples: &[SampleType]) {
    let current_peak = samples.iter().fold(0 as SampleType, |peak, &sample| {
        if sample > 0 {
            peak.max(sample.min(SampleType::MAX))
        } else {
            peak.min(sample.max(SampleType::MIN))
        }
    });

    // Отправляем событие о прогрессе, если нужно
    if let Some(channel) = get_event_channel_record() {
        let mut last_send = LAST_SEND_TIME.lock().unwrap();
        if last_send.elapsed() >= THROTTLE_DURATION {
            channel
                .send(RecordEvent::Progress {
                    timestamp: get_current_time_millis(),
                    peak: current_peak,
                })
                .ok();

            *last_send = Instant::now();
        }
    }
}

/// Записывает сэмплы в WAV файл
fn write_to_wav(samples: &[SampleType], writer: &WavWriterHandle) {
    let writer_clone = writer.clone();
    let samples = samples.to_vec();

    std::thread::spawn(move || {
        if let Ok(mut guard) = writer_clone.lock() {
            if let Some(writer) = guard.as_mut() {
                for &sample in samples.iter() {
                    if let Err(e) = writer.write_sample(sample) {
                        eprintln!("Ошибка записи в WAV файл: {}", e);
                        break;
                    }
                }
            }
        } else {
            eprintln!("Не удалось получить доступ к WAV writer");
        }
    });
}

/// Отправляет сэмплы в WhisperStreamer
fn send_to_whisper(samples: &[SampleType]) {
    let samples = samples.to_vec();
    std::thread::spawn(move || {
        if let Err(e) = WhisperStreamer::send_audio(&samples) {
            eprintln!("Ошибка отправки аудио в WhisperStreamer: {}", e);
        }
    });
}

/// Обработчик аудио данных - конвертирует входные данные и распределяет их по обработчикам
fn handle_audio_data<T>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    SampleType: Sample + FromSample<T>,
{
    // Конвертируем входные данные
    let samples: Vec<SampleType> = input
        .iter()
        .map(|&sample| SampleType::from_sample(sample))
        .collect();

    // Добавляем отладочную информацию о входных данных
    if DEBUG_AUDIO.load(Ordering::SeqCst) {
        println!("Input sample count: {}", input.len());
        println!("Converted sample count: {}", samples.len());

        // Выводим информацию о первых нескольких сэмплах
        if !samples.is_empty() {
            println!("First 5 samples: {:?}", &samples[..samples.len().min(5)]);
        }
    }

    // Запись в файл
    write_to_wav(&samples, writer);

    // Обработка пиков
    process_peaks(&samples);

    // Отправка в Whisper с отладочной информацией
    match WhisperStreamer::send_audio(&samples) {
        Ok(_) => {
            if DEBUG_AUDIO.load(Ordering::SeqCst) {
                println!("Successfully processed audio chunk");
            }
        }
        Err(e) => {
            eprintln!("Warning: Failed to send audio to Whisper: {}", e);
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

/// Создает аудио стрим для конкретного типа данных
fn build_typed_input_stream<T>(device_id: &str) -> Result<(cpal::Stream, WavWriterHandle)>
where
    T: Sample + Send + SizedSample + 'static,
    SampleType: Sample + FromSample<T>,
{
    // Получаем устройство и конфигурацию
    let device = get_input_device(device_id)?;
    let config = device
        .default_input_config()
        .map_err(|e| anyhow::anyhow!("Ошибка получения конфигурации: {}", e))?;

    // Создаем WAV writer
    let spec = create_wav_spec(&config);
    let writer = WavWriter::create(RECORDING_PATH, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));
    let writer_clone = writer.clone();

    // Создаем обработчик ошибок
    let err_fn = move |err| handle_stream_error(err);

    // Создаем поток
    let stream = device
        .build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    handle_audio_data::<T>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )
        .map_err(|e| anyhow::anyhow!("Ошибка создания потока: {}", e))?;

    Ok((stream, writer))
}

/// Записывает аудио с выбранного устройства
pub fn record(device_id: &str) -> Result<()> {
    // Включаем отладку
    // set_audio_debug(true);
    // WhisperStreamer::set_debug(true);

    // Начинаем подключение к Whisper асинхронно
    if let Err(e) = WhisperStreamer::initialize() {
        eprintln!("Warning: Failed to initialize Whisper connection: {}", e);
        // Продолжаем запись даже при ошибке подключения
    }

    if RECORDING_ACTIVE.load(Ordering::SeqCst) {
        return Err(anyhow::anyhow!("Запись уже идет"));
    }

    // Получаем конфигурацию для определения формата
    let device = get_input_device(device_id)?;
    let config = device.default_input_config()?;

    // Создаем поток записи в зависимости от формата
    let (stream, writer) = match config.sample_format() {
        cpal::SampleFormat::I8 => build_typed_input_stream::<i8>(device_id)?,
        cpal::SampleFormat::I16 => build_typed_input_stream::<i16>(device_id)?,
        cpal::SampleFormat::I32 => build_typed_input_stream::<i32>(device_id)?,
        cpal::SampleFormat::F32 => build_typed_input_stream::<f32>(device_id)?,
        format => return Err(anyhow::anyhow!("Неподдерживаемый формат: {format}")),
    };

    // Отправка события о начале записи
    if let Some(channel) = get_event_channel_record() {
        channel
            .send(RecordEvent::Start {
                timestamp: get_current_time_millis(),
            })
            .ok();
    }

    println!("Начало записи в файл: {}", RECORDING_PATH);

    // Устанавливаем флаг активной записи
    RECORDING_ACTIVE.store(true, Ordering::SeqCst);

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
pub fn stop() -> Result<String> {
    if !RECORDING_ACTIVE.load(Ordering::SeqCst) {
        return Ok(String::new());
    }

    // Закрываем соединение с Whisper сервером и получаем текст
    let transcribed_text = WhisperStreamer::close()?;

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

    if let Some(channel) = get_event_channel_record() {
        channel
            .send(RecordEvent::Stop {
                timestamp: get_current_time_millis(),
            })
            .ok();
    }
    println!("Запись остановлена");

    Ok(transcribed_text)
}

/// Добавляем публичную функцию для управления режимом отладки
pub fn set_audio_debug(enable: bool) {
    DEBUG_AUDIO.store(enable, Ordering::SeqCst);
}
