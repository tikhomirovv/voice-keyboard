mod modules;
use lazy_static::lazy_static;
use modules::{
    audio::{get_microphones as get_audio_microphones, record, stop},
    events::RecordEvent,
    input::paste_text,
    transcribation::local::inference,
};
use std::sync::Mutex;
use tauri::ipc::Channel;

lazy_static! {
    static ref GLOBAL_EVENT_CHANNEL: Mutex<Option<Channel<RecordEvent>>> = Mutex::new(None);
}

#[tauri::command]
fn start_record(device_id: &str, on_event: Channel<RecordEvent>) {
    // Сохраняем канал глобально
    *GLOBAL_EVENT_CHANNEL.lock().unwrap() = Some(on_event.clone());
    let _ = record(device_id);
}

#[tauri::command]
fn stop_record() {
    let _ = stop();
}

#[tauri::command]
fn start_transcribation() -> String {
    let result = inference();
    let _ = paste_text(&result);
    result
}

// Функция-обертка для Tauri
#[tauri::command]
fn get_microphones() -> Result<String, String> {
    get_audio_microphones().map_err(|e| format!("Ошибка получения микрофонов: {:?}", e))
}

// Функция для получения канала из других модулей
pub fn get_event_channel() -> Option<Channel<RecordEvent>> {
    GLOBAL_EVENT_CHANNEL.lock().unwrap().clone()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Создаём Tauri приложение и связываем команды
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_microphones,
            start_record,
            stop_record,
            start_transcribation,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
