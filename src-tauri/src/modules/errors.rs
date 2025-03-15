use crate::modules::events::message::MessageEvent;

/// Коды ошибок приложения
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ErrorCode {
    // Configuration errors (0-9)
    // ConfigError = 0,

    // Connection errors (10-19)
    ConnectionError = 10,
    // NotConnected = 11,

    // Stream errors (20-29)
    StreamError = 20,
    WriteError = 21,
    ReadError = 22,
    // System errors (30-39)
    // AppHandleNotInitialized = 30,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            // ErrorCode::ConfigError => "CONFIG_ERROR",
            ErrorCode::ConnectionError => "CONNECTION_ERROR",
            // ErrorCode::NotConnected => "NOT_CONNECTED",
            ErrorCode::StreamError => "STREAM_ERROR",
            ErrorCode::WriteError => "WRITE_ERROR",
            ErrorCode::ReadError => "READ_ERROR",
            // ErrorCode::AppHandleNotInitialized => "APP_HANDLE_NOT_INITIALIZED",
        }
    }
}

/// Структура для отправки событий об ошибках
pub struct ErrorEmitter;

impl ErrorEmitter {
    /// Отправить событие об ошибке в фронтенд
    pub fn emit_error(code: ErrorCode, message: &str) {
        println!("[ErrorEmitter] {} - {}", code.as_str(), message);
        MessageEvent::error(code as u8, message).send();
    }
}
