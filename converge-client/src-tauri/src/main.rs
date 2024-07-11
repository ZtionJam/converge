#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, Mutex};

use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};
use util::{open_main_window, AppState};
use window_shadows::set_shadow;
mod action;
mod util;
use tauri::SystemTrayEvent::*;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tokio::main]
async fn main() {
    //tray
    let status = CustomMenuItem::new("Status".to_string(), "状态:未连接");
    let quit = CustomMenuItem::new("Quit".to_string(), "关闭");
    let hide = CustomMenuItem::new("Open".to_string(), "打开主界面");
    let tray_menu = SystemTrayMenu::new()
        .add_item(status)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(hide);

    let tray = SystemTray::new().with_menu(tray_menu).with_tooltip("焦距");
    //channel
    let app_state = Arc::new(Mutex::new(AppState {
        current_channel: None,
        msgs: Vec::new(),
    }));

    tauri::Builder::default()
        .manage(app_state)
        .setup(|app| {
            let main_window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();
            Ok(())
        })
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            MenuItemClick { id, .. } => {
                let id = id.as_str();
                match id {
                    "Open" => open_main_window(app),
                    "Quit" => app.exit(0),
                    _ => {}
                }
            }
            LeftClick { .. } => {
                open_main_window(app);
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            action::setting,
            action::connect,
            action::get_history,
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
