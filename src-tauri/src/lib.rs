mod commands;
mod modules;
mod utils;
use lazy_static::lazy_static;
use std::sync::Arc;
use std::sync::Mutex;
use tauri::AppHandle;

lazy_static! {
    static ref GLOBAL_APP_HANDLE: Mutex<Option<Arc<AppHandle>>> = Mutex::new(None);
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
            eprintln!("[Error] Failed to acquire app handle lock");
            None
        }
    }
}

fn setup_app(app: &mut tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // Инициализируем глобальный AppHandle
    init_app_handle(app.handle().clone());
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Создаём Tauri приложение и связываем команды
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .format(|out, message, record| {
                    out.finish(format_args!(
                        "[{}] {}",
                        record.level(),
                        // record.target(),
                        message
                    ))
                })
                .build(),
        )
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_microphones,
            commands::start_record,
            commands::stop_record,
            // commands::start_transcribation,
            commands::set_event_channel_record,
            commands::get_monitor_info,
        ])
        .setup(setup_app)
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
