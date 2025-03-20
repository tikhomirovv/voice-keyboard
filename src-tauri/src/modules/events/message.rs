use crate::app::get_app_handle;
use crate::utils::get_current_timestamp;
use serde::Serialize;
use tauri::Emitter;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "type", content = "data")]
pub enum MessageEvent {
    #[serde(rename_all = "camelCase")]
    Error {
        code: u8,
        code_str: String,
        message: String,
        timestamp: u64,
    },
}

impl MessageEvent {
    const EVENT_NAME: &str = "message";
    pub fn error(code: u8, message: &str) -> Self {
        MessageEvent::Error {
            code,
            code_str: code.to_string(),
            message: message.to_string(),
            timestamp: get_current_timestamp(),
        }
    }
    pub fn send(&self) {
        let app_handle = get_app_handle().unwrap();
        app_handle.emit(Self::EVENT_NAME, self).unwrap();
    }
}
