use smart_hosts_bridge::NetworkEvent;
use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    AppHandle,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn monitor(_app: AppHandle) -> NetworkEvent {
    use NetworkEvent::*;

    //use tauri::Manager;
    //use tauri_plugin_positioner::{Position, WindowExt};

    //let mut win = app.get_webview_window("main").unwrap();
    //let _ = win.as_ref().window().move_window(Position::TrayCenter);

    Unknown
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![monitor])
        .setup(|app| {
            app.hide().unwrap();

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .on_tray_icon_event(|tray_handle, event| {
                    tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);
                    use tauri::Manager;
                    use tauri_plugin_positioner::{Position, WindowExt};

                    let win = tray_handle.app_handle().get_webview_window("main").unwrap();
                    let _ = win.as_ref().window().move_window(Position::TrayCenter);
                    win.show().unwrap();
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
