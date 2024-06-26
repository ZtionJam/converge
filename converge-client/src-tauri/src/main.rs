#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_shadows::set_shadow;
mod action;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet,action::setting])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
