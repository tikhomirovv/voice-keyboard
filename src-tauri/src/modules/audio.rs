use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Sample, SupportedStreamConfig,
};
use hound::{WavSpec, WavWriter};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
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
}

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

/// Записывает входные данные в WAV файл
fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
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

/// Безопасно останавливает текущую запись
pub fn stop() -> Result<()> {
    if !RECORDING_ACTIVE.load(Ordering::SeqCst) {
        return Ok(());
    }

    RECORDING_ACTIVE.store(false, Ordering::SeqCst);

    // Безопасно останавливаем поток и освобождаем ресурсы
    if let Some(s) = CURRENT_STREAM.with(|s| s.borrow_mut().take()) {
        drop(s);
    }

    if let Ok(mut writer) = CURRENT_WRITER.lock() {
        if let Some(w) = writer.take() {
            if let Some(w) = w.lock().unwrap().take() {
                w.finalize()?;
            }
        }
    }

    println!("Запись остановлена");
    Ok(())
}

/// Записывает аудио с выбранного устройства
pub fn record(device_id: &str) -> Result<()> {
    // Проверяем, не идет ли уже запись
    if RECORDING_ACTIVE.load(Ordering::SeqCst) {
        return Err(anyhow::anyhow!("Запись уже идет"));
    }

    let device = get_input_device(device_id)?;
    println!("Устройство ввода: {}", device.name()?);

    let config = device
        .default_input_config()
        .map_err(|e| anyhow::anyhow!("Ошибка получения конфигурации: {}", e))?;
    println!("Конфигурация ввода: {:?}", config);

    let spec = create_wav_spec(&config);
    let writer = WavWriter::create(RECORDING_PATH, spec)?;
    let writer = Arc::new(Mutex::new(Some(writer)));
    let writer_clone = writer.clone();

    println!("Начало записи в файл: {}", RECORDING_PATH);

    let err_fn = move |err| eprintln!("Ошибка потока: {}", err);

    // Устанавливаем флаг активной записи
    RECORDING_ACTIVE.store(true, Ordering::SeqCst);

    // Создаем поток записи
    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    write_input_data::<i8, i8>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    write_input_data::<i16, i16>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    write_input_data::<i32, i16>(data, &writer_clone)
                }
            },
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _| {
                if RECORDING_ACTIVE.load(Ordering::SeqCst) {
                    write_input_data::<f32, i16>(data, &writer_clone)
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
