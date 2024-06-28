use std::{sync::{Arc, Mutex}, time::Duration};

use lazy_static::lazy_static;
use reqwest::Client;
use tauri::{async_runtime::Receiver, AppHandle, Manager};
use tokio::{sync::{ mpsc}, time::timeout};

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    pub message: String,
}

#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Server {
    pub host: String,
    pub id: String,
    pub id2: String,
    pub notify: bool,
}

pub fn send_notify(app: AppHandle, msg: &str) {
    let _ = app.emit_all(
        "notify",
        Payload {
            message: msg.into(),
        },
    );
}

pub fn send_msg(app: AppHandle, msg: &str) {
    let _ = app.emit_all(
        "msg",
        Payload {
            message: msg.into(),
        },
    );
}

pub async fn listen_sse(app: AppHandle, server: Server) {
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

pub async fn listen(app: AppHandle, serverStr: String, mut rx:Arc<Mutex<mpsc::Receiver<String>>>) {
    // let mut url = server.host + "?id=" + &server.id;
    // if server.id2.len() > 0 {
    //     url = url + "&id2=" + &server.id2;
    // }
    // let client = Client::new();

    // let mut response = client.get(url).send().await.unwrap();

    // loop {
    //     tokio::select! {
    //         biased;

    //         _ = rx.recv() => {
    //             println!("接收到停止信号");
    //             break;
    //         }

    //         result = timeout(Duration::from_secs(5), response.chunk()) => {
    //             match result {
    //                 Ok(Ok(Some(chunk))) => {
    //                     println!("Received chunk: {:?}", chunk);
    //                     // 处理 chunk
    //                 },
    //                 Ok(Ok(None)) => {
    //                     println!("No more chunks.");
    //                     break;
    //                 },
    //                 Ok(Err(e)) => {
    //                     println!("Error reading chunk: {:?}", e);
    //                     break;
    //                 },
    //                 Err(_) => {
    //                     println!("Timeout waiting for chunk");
    //                     // 可选择此处退出或者继续尝试
    //                 }
    //             }
    //         }
    //     }
    // }
}
