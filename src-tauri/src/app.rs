use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::{AppHandle, Manager};

lazy_static! {
    static ref GLOBAL_APP_HANDLE: Mutex<Option<Arc<AppHandle>>> = Mutex::new(None);
}

#[allow(dead_code)]
pub fn is_debug() -> bool {
    let is_debug = std::env::var("IS_DEBUG").unwrap_or_else(|_| "0".to_string());
    is_debug == "1" || is_debug == "true"
}

/// Инициализация глобального AppHandle
pub fn init_app_handle(app_handle: AppHandle) {
    if let Ok(mut handle) = GLOBAL_APP_HANDLE.lock() {
        *handle = Some(Arc::new(app_handle));
    }
}

pub fn get_local_data_dir(path: &str) -> anyhow::Result<String> {
    let app = get_app_handle().unwrap();
    let local_data_dir = app
        .path()
        .resolve(path, tauri::path::BaseDirectory::AppLocalData)?;
    Ok(local_data_dir.to_str().unwrap().to_string())
}

/// Получение глобального AppHandle
pub fn get_app_handle() -> Result<Arc<AppHandle>, String> {
    match GLOBAL_APP_HANDLE.lock() {
        Ok(guard) => Ok(guard
            .clone()
            .ok_or_else(|| "App handle is not initialized".to_string())?),
        Err(_) => {
            let error_message = "[Error] Failed to acquire app handle lock".to_string();
            eprintln!("{}", error_message);
            Err(error_message)
        }
    }
}
