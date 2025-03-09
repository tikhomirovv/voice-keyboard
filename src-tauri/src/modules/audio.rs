use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    Device, FromSample, Host, Sample, SupportedStreamConfig,
};
use hound::{WavSpec, WavWriter};
use serde::{Deserialize, Serialize};
use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::BufWriter,
    sync::{Arc, Mutex},
};

const RECORDING_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../recorded.wav");
const RECORDING_DURATION_SECS: u64 = 10;

type WavWriterHandle = Arc<Mutex<Option<WavWriter<BufWriter<File>>>>>;

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

/// Записывает аудио с выбранного устройства
pub fn record(device_id: &str) -> Result<()> {
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

    let stream = match config.sample_format() {
        cpal::SampleFormat::I8 => device.build_input_stream(
            &config.into(),
            move |data, _| write_input_data::<i8, i8>(data, &writer_clone),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I16 => device.build_input_stream(
            &config.into(),
            move |data, _| write_input_data::<i16, i16>(data, &writer_clone),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::I32 => device.build_input_stream(
            &config.into(),
            move |data, _| write_input_data::<i32, i16>(data, &writer_clone),
            err_fn,
            None,
        )?,
        cpal::SampleFormat::F32 => device.build_input_stream(
            &config.into(),
            move |data, _| write_input_data::<f32, i16>(data, &writer_clone),
            err_fn,
            None,
        )?,
        format => return Err(anyhow::anyhow!("Неподдерживаемый формат: {format}")),
    };

    stream.play()?;
    std::thread::sleep(std::time::Duration::from_secs(RECORDING_DURATION_SECS));
    drop(stream);

    if let Some(writer) = writer.lock().unwrap().take() {
        writer.finalize()?;
    }

    println!("Запись завершена: {}", RECORDING_PATH);
    Ok(())
}
