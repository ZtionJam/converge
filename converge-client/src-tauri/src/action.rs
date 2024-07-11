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
    //注册mpsc，向上一个连接发送关闭消息,保存当前连接的sender
    let (tx, mut rx) = mpsc::channel::<i32>(10);
    {
        let mut state = state.lock().unwrap();
        if let Some(ref mut tx) = state.current_channel {
            let _ = tx.send(1);
        }
        state.current_channel = Some(tx);
    }
    //开始发起连接
    let client = Client::builder()
        .timeout(Duration::from_secs(864000))
        .build()
        .unwrap();
    let mut response = match client.get(server.get_url()).send().await {
        Ok(res) => res,
        Err(_) => {
            send_notify(app, "❌链接失败,请检查设置");
            panic!("connection err");
        }
    };
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
                      let str=  &String::from_utf8_lossy(&chunk).to_string().replace("data:", "").trim().to_string();
                      println!("消息:>>{}<<<",str);

                      if let Ok(msg)=serde_json::from_str::<Msg>(str){
                        {
                            let mut state =  state.lock().unwrap();
                            state.msgs.push(msg.clone());
                        }
                        process_msg(msg,app.clone());
                      }else if "ok".eq(str) {
                        process_connection_ok(app.clone());
                      }
                    },
                    Ok(Ok(None)) => {
                        send_system_notify(&app, "链接已断开".to_string(), "与服务器的链接已断开".to_string());
                        println!("链接关闭");
                        break;
                    },
                    _=>{
                        println!("等待中");
                    }
                }
            }
        }
    }
    println!("连接已结束");

    process_connection_close(app.clone());

    Ok(())
}
#[tauri::command]
pub fn get_history(state: State<'_, Arc<Mutex<AppState>>>) -> Vec<Msg> {
    let state = state.lock().unwrap();

    state.msgs.clone()
}
