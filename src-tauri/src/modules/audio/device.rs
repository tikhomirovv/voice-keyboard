use anyhow::Result;
use cpal::{
    traits::{DeviceTrait, HostTrait},
    Device,
};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

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
pub fn get_input_device(device_id: &str) -> Result<Device> {
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
