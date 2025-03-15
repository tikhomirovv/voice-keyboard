mod commands;
mod modules;
use lazy_static::lazy_static;
use modules::errors;
use modules::events::RecordEvent;
use std::sync::Mutex;
use tauri::ipc::Channel;

lazy_static! {
    static ref GLOBAL_EVENT_CHANNEL_RECORD: Mutex<Option<Channel<RecordEvent>>> = Mutex::new(None);
}

// Функция для получения канала из других модулей
pub fn get_event_channel_record() -> Option<Channel<RecordEvent>> {
    GLOBAL_EVENT_CHANNEL_RECORD.lock().unwrap().clone()
}

// Функция для установки канала
pub fn set_event_channel_record_global(channel: Channel<RecordEvent>) {
    *GLOBAL_EVENT_CHANNEL_RECORD.lock().unwrap() = Some(channel);
}

fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Инициализируем глобальный AppHandle
    errors::init_app_handle(app.handle().clone());
    Ok(())
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
            commands::get_microphones,
            commands::start_record,
            commands::stop_record,
            commands::start_transcribation,
            commands::set_event_channel_record,
            commands::get_monitor_info,
        ])
        .setup(setup_app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
