use crate::modules::errors::{ErrorCode, ErrorEmitter};
use anyhow::Result;
use lazy_static::lazy_static;
use std::collections::VecDeque;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::time::Duration;

lazy_static! {
    static ref TCP_STREAM: Mutex<Option<TcpStream>> = Mutex::new(None);
    static ref ACCUMULATED_TEXT: Mutex<String> = Mutex::new(String::new());
    static ref SAMPLE_BUFFER: Mutex<VecDeque<Vec<i8>>> = Mutex::new(VecDeque::new());
    static ref IS_PROCESSING: AtomicBool = AtomicBool::new(false);
    static ref IS_CONNECTED: AtomicBool = AtomicBool::new(false);
    static ref DEBUG_WHISPER: AtomicBool = AtomicBool::new(false);
}

pub struct WhisperStreamer;

impl WhisperStreamer {
    fn get_connection_string() -> Result<String> {
        let host = env::var("WHISPER_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("WHISPER_PORT").map_or(43001, |p| p.parse().unwrap_or(43001));

        Ok(format!("{}:{}", host, port))
    }

    /// Инициализация соединения с сервером Whisper
    pub fn initialize() -> Result<()> {
        // Очищаем состояние
        if let Ok(mut text) = ACCUMULATED_TEXT.lock() {
            text.clear();
        }
        if let Ok(mut buffer) = SAMPLE_BUFFER.lock() {
            buffer.clear();
        }
        IS_CONNECTED.store(false, Ordering::SeqCst);

        let address = Self::get_connection_string()?;

        // Запускаем подключение в отдельном потоке
        std::thread::spawn(move || {
            match TcpStream::connect(&address) {
                Ok(stream) => {
                    let reader_stream = match stream.try_clone() {
                        Ok(s) => s,
                        Err(e) => {
                            ErrorEmitter::emit_error(
                                ErrorCode::StreamError,
                                &format!("Failed to clone TCP stream: {}", e),
                            );
                            return;
                        }
                    };

                    // Устанавливаем соединение
                    if let Ok(mut guard) = TCP_STREAM.lock() {
                        *guard = Some(stream);
                        IS_CONNECTED.store(true, Ordering::SeqCst);
                    }

                    // Запускаем обработчик буфера
                    Self::start_buffer_processing();

                    // Запускаем чтение ответов
                    let reader = BufReader::new(reader_stream);
                    for line in reader.lines() {
                        match line {
                            Ok(text) => {
                                if let Ok(mut accumulated) = ACCUMULATED_TEXT.lock() {
                                    accumulated.push_str(&text);
                                    accumulated.push('\n');
                                }
                            }
                            Err(e) => {
                                ErrorEmitter::emit_error(
                                    ErrorCode::ReadError,
                                    &format!("Error reading from server: {}", e),
                                );
                                break;
                            }
                        }
                    }
                }
                Err(e) => {
                    ErrorEmitter::emit_error(
                        ErrorCode::ConnectionError,
                        &format!("Failed to connect to Whisper server at {}: {}", address, e),
                    );
                }
            }
        });

        Ok(())
    }

    // Новый метод для обработки буфера
    fn start_buffer_processing() {
        IS_PROCESSING.store(true, Ordering::SeqCst);

        std::thread::spawn(move || {
            while IS_PROCESSING.load(Ordering::SeqCst) {
                // Получаем данные из буфера
                let samples = {
                    let mut buffer = SAMPLE_BUFFER.lock().unwrap();
                    buffer.pop_front()
                };

                if let Some(samples) = samples {
                    // Отправляем данные, если есть соединение
                    if let Ok(mut guard) = TCP_STREAM.lock() {
                        if let Some(ref mut stream) = *guard {
                            let raw_bytes = unsafe {
                                std::slice::from_raw_parts(
                                    samples.as_ptr() as *const u8,
                                    samples.len(),
                                )
                            };
                            if let Err(e) = stream.write_all(raw_bytes) {
                                ErrorEmitter::emit_error(
                                    ErrorCode::WriteError,
                                    &format!("Failed to send audio data: {}", e),
                                );
                                break;
                            }
                        }
                    }
                } else {
                    // Если буфер пуст, делаем паузу
                    std::thread::sleep(Duration::from_millis(10));
                }
            }
        });
    }

    pub fn set_debug(enable: bool) {
        DEBUG_WHISPER.store(enable, Ordering::SeqCst);
    }

    fn validate_audio_data(samples: &[i8]) -> Result<()> {
        if DEBUG_WHISPER.load(Ordering::SeqCst) {
            // Проверяем базовые параметры
            if samples.is_empty() {
                return Err(anyhow::anyhow!("Empty audio sample"));
            }

            // Проверяем размер данных
            println!("Audio data size: {} bytes", samples.len());

            // Проверяем диапазон значений и считаем статистику
            let mut min_value = i8::MAX;
            let mut max_value = i8::MIN;
            let mut sum: i32 = 0;
            let mut zero_count = 0;

            for &sample in samples {
                min_value = min_value.min(sample);
                max_value = max_value.max(sample);
                sum += sample as i32;
                if sample == 0 {
                    zero_count += 1;
                }
            }

            let avg = sum as f32 / samples.len() as f32;

            println!("Audio statistics:");
            println!("  Min value: {}", min_value);
            println!("  Max value: {}", max_value);
            println!("  Average: {:.2}", avg);
            println!(
                "  Zero samples: {} ({:.2}%)",
                zero_count,
                (zero_count as f32 * 100.0) / samples.len() as f32
            );

            // Проверяем на подозрительные паттерны
            if zero_count == samples.len() {
                println!("WARNING: All samples are zero!");
            }

            if max_value - min_value < 10 {
                println!("WARNING: Very low dynamic range!");
            }
        }

        Ok(())
    }

    /// Отправка аудио данных на сервер
    pub fn send_audio(samples: &[i8]) -> Result<()> {
        // Сначала валидируем данные
        if let Err(e) = Self::validate_audio_data(samples) {
            eprintln!("Audio validation failed: {}", e);
            return Err(e);
        }

        // Существующий код отправки данных...
        if let Ok(mut guard) = TCP_STREAM.lock() {
            if let Some(ref mut stream) = *guard {
                if DEBUG_WHISPER.load(Ordering::SeqCst) {
                    println!("Sending {} bytes to Whisper server", samples.len());
                }

                let raw_bytes = unsafe {
                    std::slice::from_raw_parts(samples.as_ptr() as *const u8, samples.len())
                };
                if let Err(e) = stream.write_all(raw_bytes) {
                    ErrorEmitter::emit_error(
                        ErrorCode::WriteError,
                        &format!("Failed to send audio data: {}", e),
                    );
                    return Err(e.into());
                }

                if DEBUG_WHISPER.load(Ordering::SeqCst) {
                    println!("Successfully sent data to Whisper server");
                }
            } else {
                return Err(anyhow::anyhow!("No active connection"));
            }
        }
        Ok(())
    }

    /// Закрытие соединения и возврат накопленного текста
    pub fn close() -> Result<String> {
        IS_PROCESSING.store(false, Ordering::SeqCst);
        IS_CONNECTED.store(false, Ordering::SeqCst);

        if let Ok(mut guard) = TCP_STREAM.lock() {
            *guard = None;
        }

        let result = if let Ok(text) = ACCUMULATED_TEXT.lock() {
            text.clone()
        } else {
            String::new()
        };

        if let Ok(mut text) = ACCUMULATED_TEXT.lock() {
            text.clear();
        }

        if let Ok(mut buffer) = SAMPLE_BUFFER.lock() {
            buffer.clear();
        }

        Ok(result)
    }

    // Добавляем новый метод для проверки соединения
    pub fn is_connected() -> bool {
        IS_CONNECTED.load(Ordering::SeqCst)
    }
}
