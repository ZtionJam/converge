use reqwest::Client;
use tauri::{self, http::Uri, AppHandle, Manager, WindowBuilder, WindowUrl};
use window_shadows::set_shadow;

use util::*;

use crate::util;

#[tauri::command]
pub async fn setting(handle: AppHandle) {
    //config
    let mut config = handle.config().tauri.windows.get(0).unwrap().clone();
    config.label = "setting".to_string();
    config.title = "设置".to_string();
    config.height = 350.0;
    config.center = false;
    config.url = WindowUrl::App("/#/setting".parse().unwrap());

    let setting_window = match WindowBuilder::from_config(&handle, config).build() {
        Ok(w) => w,
        Err(e) => {
            if e.to_string().contains("exists") {
                let _ = handle.emit_all(
                    "notify",
                    Payload {
                        message: "✔设置窗口已经打开了哦".into(),
                    },
                );
                if let Some(win) = handle.get_window("setting") {
                    let _ = win.set_focus();
                    return;
                }
            }
            panic!("open setting err")
        }
    };
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&setting_window, true).unwrap();
}

#[tauri::command]
pub async fn connect(app: AppHandle, server: Server) {
    let mut url = server.host + "?id=" + &server.id;
    if server.id2.len() > 0 {
        url = url + "&id2=" + &server.id2;
    }
    let client = Client::new();
    
    let mut response = client.get(url).send().await.unwrap();

    let mut msgBuff = String::new();
    while let Some(item) = response.chunk().await.unwrap() {
        msgBuff = msgBuff + &String::from_utf8_lossy(&item).to_string();
        if msgBuff.ends_with("\n\n") {
            if msgBuff.starts_with("data") {
                println!("Received: {}", msgBuff);
                if msgBuff.trim().eq("data:ok") {
                    send_notify(app.clone(), "✔链接成功");
                } else {
                    send_msg(app.clone(), &msgBuff.trim().replace("data:", ""));
                }
            }

            msgBuff.clear();
        }
    }
    send_notify(app, "❌链接已断开");
}
