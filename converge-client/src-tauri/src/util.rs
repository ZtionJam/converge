use tauri::api::notification::Notification;
use tauri::{AppHandle, Manager, WindowBuilder};
use tokio::sync::mpsc;
use window_shadows::set_shadow;

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
#[derive(Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct Msg {
    pub content: String,
    pub id: String,
    pub id2: String,
}
impl Server {
    pub fn get_url(&self) -> String {
        return {
            let mut first = self.host.clone() + "?id=" + self.id.as_str();
            if self.id2.len() > 0 {
                first = first + "&id2=" + self.id2.as_str();
            }
            first
        };
    }
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

//处理收到的信息
pub fn process_msg(msg: Msg, app: AppHandle) {
    send_system_notify(&app, "收到新消息".to_string(), msg.content.clone());
    send_msg(app.clone(), serde_json::to_string(&msg).unwrap().as_str());
}

//处理连接成功事件
pub fn process_connection_ok(app: AppHandle) {
    let _ = app
        .tray_handle()
        .get_item("Status")
        .set_title("状态：已连接");
    send_notify(app.clone(), "✔已连接成功");
}
//处理连接断开事件
pub fn process_connection_close(app: AppHandle) {
    let _ = app
        .tray_handle()
        .get_item("Status")
        .set_title("状态：未连接");
    send_notify(app, "❌链接断开");
}

pub fn open_main_window(app: &AppHandle) {
    if let None = app.get_window("main") {
        let windows_config = app.config().tauri.windows.get(0).unwrap().clone();
        let window = WindowBuilder::from_config(app, windows_config)
            .build()
            .unwrap();
        #[cfg(any(windows, target_os = "macos"))]
        set_shadow(&window, true).unwrap();
    } else {
        let _ = app.get_window("main").unwrap().set_focus();
    }
}

pub fn send_system_notify(app: &AppHandle, title: String, body: String) {
    let _ = Notification::new(app.config().tauri.bundle.identifier.clone())
        .title(title)
        .body(body)
        .show();
}

pub struct AppState {
    pub current_channel: Option<mpsc::Sender<i32>>,

    pub msgs: Vec<Msg>,
}
