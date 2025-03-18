pub mod device;
pub mod session;
pub mod wav_writer;

pub type SampleType = i8;

use crate::modules::{
    audio::{device::get_input_device, session::RecordingSession, wav_writer::AudioFileWriter},
    events::record::RecordEvent,
};
use anyhow::Result;
use cpal::traits::DeviceTrait;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::broadcast::{self, error::RecvError};
use tokio::sync::Mutex;

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
        session.stop();
        println!("Сессия остановлена.");
        RecordEvent::stop().send();
    }

    println!("Остановка записи");
    Ok(String::new())
}

// Следит за временем записи и останавливает её при превышении лимита
async fn watch_recording_time() {
    let mut elapsed = 0u64;
    let recording_active = CURRENT_SESSION.lock().await.is_some();
    while recording_active && elapsed < MAX_RECORDING_DURATION_SECS {
        tokio::time::sleep(Duration::from_secs(1)).await;
        elapsed += 1;
    }

    // Если запись все еще активна, значит достигнут лимит времени
    if recording_active {
        println!(
            "Достигнут максимальный лимит записи ({} секунд)",
            MAX_RECORDING_DURATION_SECS
        );
        let _ = stop().await;
    }
}

async fn send_peaks(mut peaks_rx: broadcast::Receiver<Vec<SampleType>>) {
    let mut last_send_time = Instant::now();
    const THROTTLE_DURATION: Duration = Duration::from_millis(10);

    while let Ok(samples) = peaks_rx.recv().await {
        let current_peak = samples.iter().fold(0 as SampleType, |peak, &sample| {
            if sample > 0 {
                peak.max(sample.min(SampleType::MAX))
            } else {
                peak.min(sample.max(SampleType::MIN))
            }
        });
        if last_send_time.elapsed() >= THROTTLE_DURATION {
            RecordEvent::progress(current_peak).send();
            last_send_time = Instant::now();
        }
    }
}

async fn write_to_wav(
    mut wav_rx: broadcast::Receiver<Vec<SampleType>>,
    sample_rate: u32,
    id: String,
) {
    // Создаем WAV файл
    let mut writer = match AudioFileWriter::create(id, sample_rate) {
        Ok(writer) => writer,
        Err(e) => {
            eprintln!("Ошибка создания WAV файла: {}", e);
            return;
        }
    };
    loop {
        match wav_rx.recv().await {
            Ok(samples) => {
                tokio::time::sleep(Duration::from_millis(100)).await;
                if let Err(e) = writer.write_samples(&samples) {
                    eprintln!("Ошибка записи в WAV: {}", e);
                    break;
                }
            }
            Err(RecvError::Lagged(skipped)) => {
                println!("Пропущено {} сэмплов из-за отставания", skipped);
                continue; // продолжаем работу
            }
            Err(RecvError::Closed) => {
                println!("Канал закрыт, завершаем запись");
                break;
            }
        }
    }
    // Закрываем файл
    if let Err(e) = writer.finalize() {
        eprintln!("Ошибка закрытия WAV файла: {}", e);
    }
    println!("WAV запись завершена");
}
