use crate::modules::errors::{ErrorCode, ErrorEmitter};
use anyhow::Result;
use lazy_static::lazy_static;
use std::env;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;
use std::sync::Mutex;

lazy_static! {
    static ref TCP_STREAM: Mutex<Option<TcpStream>> = Mutex::new(None);
    static ref ACCUMULATED_TEXT: Mutex<String> = Mutex::new(String::new());
}

pub struct WhisperStreamer;

impl WhisperStreamer {
    fn get_connection_string() -> Result<String> {
        let host = env::var("WHISPER_HOST").unwrap_or_else(|_| "localhost".to_string());
        let port = env::var("WHISPER_PORT")
            .unwrap_or_else(|_| "43001".to_string())
            .parse::<u16>()
            .unwrap_or(43001);

        Ok(format!("{}:{}", host, port))
    }

    /// Инициализация соединения с сервером Whisper
    pub fn initialize() -> Result<()> {
        // Очищаем накопленный текст при старте новой сессии
        if let Ok(mut text) = ACCUMULATED_TEXT.lock() {
            text.clear();
        }

        let address = match Self::get_connection_string() {
            Ok(addr) => addr,
            Err(e) => {
                ErrorEmitter::emit_error(
                    ErrorCode::ConfigError,
                    &format!("Failed to get connection string: {}", e),
                );
                return Err(e);
            }
        };

        let stream = match TcpStream::connect(&address) {
            Ok(s) => s,
            Err(e) => {
                ErrorEmitter::emit_error(
                    ErrorCode::ConnectionError,
                    &format!("Failed to connect to Whisper server at {}: {}", address, e),
                );
                return Err(e.into());
            }
        };

        let reader_stream = match stream.try_clone() {
            Ok(s) => s,
            Err(e) => {
                ErrorEmitter::emit_error(ErrorCode::StreamError, "Failed to clone TCP stream");
                return Err(e.into());
            }
        };

        if let Ok(mut guard) = TCP_STREAM.lock() {
            *guard = Some(stream);
        }

        // Запускаем чтение ответов в отдельном потоке
        std::thread::spawn(move || {
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
        });

        Ok(())
    }

    /// Отправка аудио данных на сервер
    pub fn send_audio(samples: &[i8]) -> Result<()> {
        if let Ok(mut guard) = TCP_STREAM.lock() {
            if let Some(ref mut stream) = *guard {
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
            } else {
                // ErrorEmitter::emit_error(
                //     ErrorCode::NotConnected,
                //     "No active connection to Whisper server",
                // );
                return Err(anyhow::anyhow!("No active connection"));
            }
        }
        Ok(())
    }

    /// Закрытие соединения и возврат накопленного текста
    pub fn close() -> Result<String> {
        // Закрываем соединение
        if let Ok(mut guard) = TCP_STREAM.lock() {
            *guard = None;
        }

        // Получаем накопленный текст
        let result = if let Ok(text) = ACCUMULATED_TEXT.lock() {
            text.clone()
        } else {
            String::new()
        };

        // Очищаем накопленный текст
        if let Ok(mut text) = ACCUMULATED_TEXT.lock() {
            text.clear();
        }

        Ok(result)
    }
}
