use crate::app::get_local_data_dir;
use crate::modules::audio::SampleType;
use crate::utils::get_current_timestamp;
use anyhow::Result;
use hound::{WavSpec, WavWriter};
use lazy_static::lazy_static;
use std::{fs::File, io::BufWriter, path::PathBuf};
use tokio::{
    sync::broadcast::{self, error::RecvError},
    sync::watch,
    time::{timeout, Duration},
};

const BITS_PER_SAMPLE: u16 = 8;

#[derive(Clone, Debug)]
pub enum WavEvent {
    RecordingComplete { id: String, path: String },
    RecordingError { id: String, error: String },
    None,
}

// Создаем глобальный канал
lazy_static! {
    static ref WAV_EVENTS: watch::Sender<WavEvent> = {
        let (sender, _) = watch::channel(WavEvent::None);
        sender
    };
}

// Функция для отправки события
fn send_wav_event(event: WavEvent) {
    if let Err(e) = WAV_EVENTS.send(event) {
        // Отсутствие подписчиков тоже считается ошибкой
        // eprintln!("Ошибка отправки события WAV: {}", e);
    }
}

pub struct AudioFileWriter {
    writer: WavWriter<BufWriter<File>>,
    id: String,
    path: PathBuf,
}

impl AudioFileWriter {
    pub fn create(id: String, sample_rate: u32) -> Result<Self> {
        let path = Self::generate_wav_path(id.clone())?;
        let writer = WavWriter::create(
            &path,
            WavSpec {
                channels: 1,
                sample_rate, // Частота дискретизации
                bits_per_sample: BITS_PER_SAMPLE,
                sample_format: hound::SampleFormat::Int,
            },
        )?;

        Ok(Self { writer, id, path })
    }

    /// Записывает блок сэмплов в файл
    pub fn write_samples(&mut self, samples: &[SampleType]) -> Result<()> {
        for &sample in samples {
            self.writer.write_sample(sample)?;
        }
        Ok(())
    }

    /// Завершает запись и закрывает файл
    pub fn finalize(self) -> Result<()> {
        self.writer.finalize()?;

        // Отправляем событие о завершении записи
        send_wav_event(WavEvent::RecordingComplete {
            path: self.path.to_string_lossy().to_string(),
            id: self.id,
        });

        Ok(())
    }

    fn generate_wav_path(id: String) -> Result<PathBuf> {
        let timestamp = get_current_timestamp();
        let wav_path = format!("{}/{}_{}.wav", "records", timestamp, id);
        let full_path = get_local_data_dir(&wav_path)?;
        let path = PathBuf::from(full_path);
        // Создаем директорию для записей если её нет
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent)?;
        }
        println!("path: {}", path.display());
        Ok(path)
    }
}

pub async fn write_to_wav(
    mut wav_rx: broadcast::Receiver<Vec<SampleType>>,
    sample_rate: u32,
    id: String,
) {
    // Создаем WAV файл
    let mut writer = match AudioFileWriter::create(id.clone(), sample_rate) {
        Ok(writer) => writer,
        Err(e) => {
            eprintln!("Ошибка создания WAV файла: {}", e);
            send_wav_event(WavEvent::RecordingError {
                id: id.clone(),
                error: format!("Ошибка создания WAV файла: {}", e),
            });
            return;
        }
    };
    loop {
        match wav_rx.recv().await {
            Ok(samples) => {
                // tokio::time::sleep(Duration::from_millis(25)).await;
                if let Err(e) = writer.write_samples(&samples) {
                    eprintln!("Ошибка записи в WAV: {}", e);
                    send_wav_event(WavEvent::RecordingError {
                        id: id.clone(),
                        error: format!("Ошибка записи в WAV: {}", e),
                    });
                    break;
                }
            }
            Err(RecvError::Lagged(skipped)) => {
                println!("Пропущено {} сэмплов из-за отставания", skipped);
                continue;
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
        send_wav_event(WavEvent::RecordingError {
            id: id.clone(),
            error: format!("Ошибка закрытия WAV файла: {}", e),
        });
    }
    println!("WAV запись завершена");
}

/// Прослушивает события завершения записи WAV файла
/// F - замыкание, которое будет вызвано при получении события
pub async fn listen_for_completion<F>(timeout_secs: u64, callback: F) -> Result<()>
where
    F: FnOnce(String, String) + Send + 'static,
{
    let mut rx = WAV_EVENTS.subscribe();
    println!("Начало прослушивания событий WAV");

    // Ждем изменения значения
    match timeout(Duration::from_secs(timeout_secs), rx.changed()).await {
        Ok(Ok(_)) => match rx.borrow().clone() {
            WavEvent::RecordingComplete { id, path } => {
                println!("Получено событие WAV: запись завершена");
                callback(id, path);
                Ok(())
            }
            WavEvent::RecordingError { id, error } => {
                eprintln!("Получено событие WAV: ошибка записи {} - {}", id, error);
                Err(anyhow::anyhow!("Ошибка записи: {}", error))
            }
            WavEvent::None => Err(anyhow::anyhow!("Получено пустое событие")),
        },
        Ok(Err(e)) => {
            eprintln!("Ошибка получения события WAV: {}", e);
            Err(anyhow::anyhow!("Ошибка получения события: {}", e))
        }
        Err(_) => {
            eprintln!("Таймаут ожидания события WAV");
            Err(anyhow::anyhow!("Таймаут ожидания события"))
        }
    }
}
