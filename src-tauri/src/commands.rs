use crate::modules::events::record::{set_event_channel_record_global, RecordEvent};
use crate::modules::{
    audio::{device::get_microphones as get_audio_microphones, record, stop},
    // transcribation::_local::inference,
    input::paste_text,
};
use tauri::{ipc::Channel, Manager};

#[tauri::command]
pub fn set_event_channel_record(channel: Channel<RecordEvent>) {
    set_event_channel_record_global(channel);
}

#[tauri::command]
pub async fn start_record(device_id: &str) -> Result<(), String> {
    let _ = record(device_id).await;
    Ok(())
}

#[tauri::command]
pub async fn stop_record() {
    let result = stop().await;
    let _ = paste_text(&result.unwrap());
}

// #[tauri::command]
// pub fn start_transcribation() -> String {
//     let result = inference();
//     let _ = paste_text(&result);
//     result
// }

use serde::Serialize;
#[derive(Clone, Serialize)]
pub struct MonitorInfo {
    size: (u32, u32),
    position: (i32, i32),
}
#[tauri::command]
pub fn get_monitor_info(app: tauri::AppHandle) -> Result<MonitorInfo, String> {
    let main_window = app.get_webview_window("main").unwrap();
    if let Ok(Some(monitor)) = main_window.primary_monitor() {
        Ok(MonitorInfo {
            size: (monitor.size().width, monitor.size().height),
            position: (monitor.position().x, monitor.position().y),
        })
    } else {
        Err("Error".to_string())
    }
}

// Функция-обертка для Tauri
#[tauri::command]
pub fn get_microphones() -> Result<String, String> {
    get_audio_microphones().map_err(|e| format!("Ошибка получения микрофонов: {:?}", e))
}
