mod modules;
use modules::{
    audio::{get_microphones as get_audio_microphones, record, stop},
    input::paste_text,
    transcribation::local::inference,
};

#[tauri::command]
fn start_record(device_id: &str) {
    let _ = record(device_id);
}

#[tauri::command]
fn stop_record() -> String {
    let _ = stop();
    let result = inference();
    let _ = paste_text(&result);
    result
}

// Функция-обертка для Tauri
#[tauri::command]
fn get_microphones() -> Result<String, String> {
    get_audio_microphones().map_err(|e| format!("Ошибка получения микрофонов: {:?}", e))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Создаём Tauri приложение и связываем команды
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_microphones,
            start_record,
            stop_record,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
