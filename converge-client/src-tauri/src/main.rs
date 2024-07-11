#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray};
use tauri::SystemTrayEvent::*;
use window_shadows::set_shadow;

use util::{AppState, open_main_window};

mod action;
mod util;
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
#[tokio::main]
async fn main() {
    //tray
    let status = CustomMenuItem::new("Status".to_string(), "状态:未连接");
    let open = CustomMenuItem::new("Open".to_string(), "打开主界面");
    let quit = CustomMenuItem::new("Quit".to_string(), "关闭");
    let tray_menu = SystemTrayMenu::new()
        .add_item(status)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(open)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(quit);

    let tray = SystemTray::new().with_menu(tray_menu).with_tooltip("焦距");
    //state
    let app_state = Arc::new(Mutex::new(AppState {
        current_channel: None,
        msgs: Vec::new(),
    }));
    //args
    let args: Vec<String> = std::env::args().collect();
    let back_run = args.iter().any(|arg| arg.contains("back_run"));
    tauri::Builder::default()
        .manage(app_state)
        .setup(move |app| {
            let main_window = app.get_window("main").unwrap();
            #[cfg(any(windows, target_os = "macos"))]
            set_shadow(&main_window, true).unwrap();
            //sample back run
            if back_run {
                let _ = main_window.hide();
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    let _ = main_window.close();
                });
            }
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
