use reqwest::Client;
use tauri::{AppHandle, Manager};
use std::sync::Mutex;
use lazy_static::lazy_static;


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

lazy_static! {
    pub static ref CLIENT: Mutex<Client> = Mutex::new(Client::new());
}
