use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

use reqwest::Client;
use tauri::{self, AppHandle, Manager, State, WindowBuilder, WindowUrl};
use tokio::{sync::mpsc, time::timeout};
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
pub async fn connect(
    app: AppHandle,
    server: Server,
    state: State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), ()> {
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    //close other connection
    {
        let mut state = state.lock().unwrap();
        if let Some(ref mut tx) = state.current_channel {
            let _ = tx.send(1);
        }
        state.current_channel = Some(tx);
    }
    //start connection

    let client = Client::new();
    let mut response = match client.get(server.get_url()).send().await {
        Ok(res) => res,
        Err(_) => {
            send_notify(app, "❌链接失败,请检查设置");
            panic!("connection err");
        }
    };
    let mut msg_buff = String::new();
    loop {
        tokio::select! {
            biased;

            _ = rx.recv() => {
                println!("收到停止信号");
                break;
            }

            result = timeout(Duration::from_secs(5), response.chunk()) => {
                match result {
                    Ok(Ok(Some(chunk))) => {
                        msg_buff += &String::from_utf8_lossy(&chunk).to_string();
                        if is_end_of_sse(&msg_buff) {
                            println!("Received: {}", msg_buff);
                            if check_sse_data(&msg_buff,app.clone()) {
                                let json= &msg_buff.trim().replace("data:", "");
                                send_msg(app.clone(),json);
                                let msg:Msg=serde_json::from_str(json).unwrap();
                                send_system_notify(&app, "收到新消息".to_string(), msg.content);
                            }
                            msg_buff.clear();
                        }
                    },
                    Ok(Ok(None)) => {
                        send_system_notify(&app, "链接已断开".to_string(), "与服务器的链接已断开".to_string());
                        println!("链接关闭");
                        break;
                    },
                    _=>{}
                }
            }
        }
    }
    println!("连接已结束");

    let _ = app
        .tray_handle()
        .get_item("Status")
        .set_title("状态：未连接");
    send_notify(app, "❌链接断开");

    Ok(())
}
