use tauri::{self, WindowBuilder, WindowUrl};
use window_shadows::set_shadow;
#[tauri::command]
pub async fn setting(handle: tauri::AppHandle) {
    // let setting_window = tauri::WindowBuilder::new(
    //     &handle,
    //     "setting",
    //     tauri::WindowUrl::App("#/setting".parse().unwrap()),
    // )
    // .build()
    // .unwrap();
    // #[cfg(any(windows, target_os = "macos"))]
    // set_shadow(&setting_window, true).unwrap();
    // let _ = setting_window.set_maximizable(false);
    // let _ = setting_window.set_decorations(false);
    // let _ = setting_window.set_title("setting");

    //config
    let mut config = handle.config().tauri.windows.get(0).unwrap().clone();
    config.label = "setting".to_string();
    config.title = "setting".to_string();
    config.height = 350.0;
    config.center = false;
    config.url = WindowUrl::App("/#/setting".parse().unwrap());

    let setting_window = WindowBuilder::from_config(&handle, config).build().unwrap();
    #[cfg(any(windows, target_os = "macos"))]
    set_shadow(&setting_window, true).unwrap();
}
