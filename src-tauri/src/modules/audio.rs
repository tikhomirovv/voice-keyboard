pub mod device;
pub mod peaks;
pub mod session;
pub mod wav_writer;

pub type SampleType = i8;

use crate::modules::{
    audio::{
        device::get_input_device,
        peaks::send_peaks,
        session::RecordingSession,
        wav_writer::{listen_for_completion, write_to_wav},
    },
    events::record::RecordEvent,
};
use anyhow::Result;
use cpal::traits::DeviceTrait;
use lazy_static::lazy_static;
use std::sync::Arc;
use tokio::{
    sync::Mutex,
    time::{sleep, Duration},
};

// const MAX_RECORDING_DURATION_SECS: u64 = 60 * 5;
const MAX_RECORDING_DURATION_SECS: u64 = 5;

// Глобальное состояние текущей сессии
lazy_static! {
    static ref CURRENT_SESSION: Arc<Mutex<Option<RecordingSession>>> = Arc::new(Mutex::new(None));
}

/// Записывает аудио с выбранного устройства
pub async fn record(device_id: &str) -> Result<()> {
    println!("======================");
    println!("Запись c устройства {}", device_id);

    // Проверяем, нет ли уже активной сессии
    if CURRENT_SESSION.lock().await.is_some() {
        return Err(anyhow::anyhow!("Запись уже идет"));
    }

    let device = get_input_device(device_id)?;
    let config = device.default_input_config()?;
    let sample_rate = config.sample_rate().0 as u32;

    let mut session = RecordingSession::new();
    let id = &session.id;
    // Создаем подписчика для WAV записи до запуска
    let wav_rx = session.subscribe();
    tokio::spawn(write_to_wav(wav_rx, sample_rate, id.clone()));
    // Создаем подписчик для отправки пиков
    let peaks_tx = session.subscribe();
    tokio::spawn(send_peaks(peaks_tx));
    // Следим за временем записи
    tokio::spawn(watch_recording_time());

    // Запускаем запись
    session.start(&device)?;
    // Сохраняем сессию в глобальное состояние
    {
        let mut current_session = CURRENT_SESSION.lock().await;
        *current_session = Some(session);
    }

    RecordEvent::start().send();
    println!("Запись начата");
    Ok(())
}

/// Безопасно останавливает текущую запись
pub async fn stop() -> Result<String> {
    if let Some(mut session) = CURRENT_SESSION.lock().await.take() {
        // Останавливаем сессию
        session.stop();
        println!("Сессия остановлена.");
        RecordEvent::stop().send();

        // Ожидаем завершения записи файла
        tokio::spawn(listen_for_completion(5, |id, path| {
            println!("Запись {} завершена: {}", id, path);
        }));
    }

    println!("Остановка записи");
    Ok(String::new())
}

// Следит за временем записи и останавливает её при превышении лимита
async fn watch_recording_time() {
    let mut elapsed = 0u64;
    while elapsed < MAX_RECORDING_DURATION_SECS {
        // Проверяем наличие сессии в каждой итерации
        let recording_active = CURRENT_SESSION.lock().await.is_some();
        if !recording_active {
            println!("Сессия завершена, останавливаем таймер");
            break;
        }
        sleep(Duration::from_secs(1)).await;
        elapsed += 1;
    }

    // Проверяем еще раз, так как сессия могла быть остановлена во время последнего sleep
    if CURRENT_SESSION.lock().await.is_some() {
        println!(
            "Достигнут максимальный лимит записи ({} секунд)",
            MAX_RECORDING_DURATION_SECS
        );
        let _ = stop().await;
    }
}
