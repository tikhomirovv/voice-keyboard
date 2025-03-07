mod audio;
mod input;
use audio::{get_microphones as get_audio_microphones, record, stop};
mod whisper;
use whisper::inference;

use input::paste_text;
// use std::thread;
// use std::time::Duration;

// #[tauri::command]
// fn greet(name: &str) -> String {
//     // Формируем приветствие
//     let greeting = format!("Hello {} Youve been greeted from Rust", name);
//     thread::sleep(Duration::from_secs(3));
//     let _ = paste_text(name);
//     greeting
// }

#[tauri::command]
fn start_record(device_id: &str) -> String {
    let _ = record(device_id);
    let result = inference();
    let _ = paste_text(&result);
    result
}

#[tauri::command]
fn stop_record() {
    // let _ = stop();
    // inference();
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
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_record, get_microphones])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
