#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, Mutex};

use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};
use tokio::sync::{broadcast, mpsc};
use util::listen;
use window_shadows::set_shadow;
mod action;
mod util;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    //tray
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu);
    //channel
    let (tx, mut rx) = mpsc::channel::<String>(10);
    let s_tx = Arc::new(tx);
    let s_rx = Arc::new(Mutex::new(rx));
    let share_tx = Arc::clone(&s_tx);
    let share_rx = Arc::clone(&s_rx);
    tauri::Builder::default()
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();

            //disconnect
            let _ = app.listen_global("disconnect", move |_| {
                let _ = share_tx.send("1".to_string());
            });

            //connect
            let _ = app.listen_global("connect", move |e| {
                // let share_rx = Arc::clone(&s_rx);
                e.payload();
            });

            Ok(())
        })
        .system_tray(tray)
        .invoke_handler(tauri::generate_handler![
            greet,
            action::setting,
            action::connect
        ])
        .build(tauri::generate_context!())
        .unwrap()
        .run(|_app_handle, event| match event {
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
