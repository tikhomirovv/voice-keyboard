use crate::modules::events::Message;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};

/// Коды ошибок приложения
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum ErrorCode {
    // Configuration errors (0-9)
    ConfigError = 0,

    // Connection errors (10-19)
    ConnectionError = 10,
    NotConnected = 11,

    // Stream errors (20-29)
    StreamError = 20,
    WriteError = 21,
    ReadError = 22,
    // System errors (30-39)
    AppHandleNotInitialized = 30,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::ConfigError => "CONFIG_ERROR",
            ErrorCode::ConnectionError => "CONNECTION_ERROR",
            ErrorCode::NotConnected => "NOT_CONNECTED",
            ErrorCode::StreamError => "STREAM_ERROR",
            ErrorCode::WriteError => "WRITE_ERROR",
            ErrorCode::ReadError => "READ_ERROR",
            ErrorCode::AppHandleNotInitialized => "APP_HANDLE_NOT_INITIALIZED",
        }
    }
}

lazy_static! {
    static ref GLOBAL_APP_HANDLE: Mutex<Option<Arc<AppHandle>>> = Mutex::new(None);
}

/// Структура для отправки событий об ошибках
pub struct ErrorEmitter;

impl ErrorEmitter {
    const EVENT_NAME: &str = "message";

    fn get_timestamp() -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis()
    }

    /// Отправить событие об ошибке в фронтенд
    pub fn emit_error(code: ErrorCode, message: &str) {
        println!("[Error] {} - {}", code.as_str(), message);
        if let Some(app_handle) = get_app_handle() {
            let event = Message::Error {
                code: code as u8,
                code_str: code.as_str().to_string(),
                message: message.to_string(),
                timestamp: Self::get_timestamp(),
            };
            let _ = app_handle.emit(Self::EVENT_NAME, event);
        } else {
            println!("[Error] AppHandle not initialized - could not emit error event");
        }
    }
}

/// Инициализация глобального AppHandle
pub fn init_app_handle(app_handle: AppHandle) {
    if let Ok(mut handle) = GLOBAL_APP_HANDLE.lock() {
        *handle = Some(Arc::new(app_handle));
    }
}

/// Получение глобального AppHandle
pub fn get_app_handle() -> Option<Arc<AppHandle>> {
    match GLOBAL_APP_HANDLE.lock() {
        Ok(guard) => guard.clone(),
        Err(_) => {
            ErrorEmitter::emit_error(
                ErrorCode::AppHandleNotInitialized,
                "Failed to acquire app handle lock",
            );
            None
        }
    }
}
