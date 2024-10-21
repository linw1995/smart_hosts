use log::debug;
use smart_hosts::monitor::Monitor;
use smart_hosts_bridge::NetworkEvent;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn monitor(app: AppHandle) -> NetworkEvent {
    use smart_hosts::monitor::Monitor;
    use smart_hosts::monitor::NetworkInfo::*;

    let m = app.state::<Monitor>();
    debug!("permission granted: {:?}", m.is_permission_granted());
    match m.get_network_info() {
        Unknown => NetworkEvent::Unknown,
        WiFi { ssid, interface } => NetworkEvent::WiFi { ssid, interface },
        Cellular { interface } => NetworkEvent::Cellular { interface },
        Wired { interface } => NetworkEvent::Wired { interface },
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![monitor])
        .setup(|app| {
            app.hide().unwrap();

            let monitor = Monitor::default();
            monitor.start();
            app.manage(monitor);

            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&quit_i])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .menu_on_left_click(true)
                .on_tray_icon_event(|tray_handle, event| {
                    use tauri_plugin_positioner::{Position, WindowExt};

                    tauri_plugin_positioner::on_tray_event(tray_handle.app_handle(), &event);

                    let win = tray_handle.app_handle().get_webview_window("main").unwrap();
                    match event {
                        TrayIconEvent::Enter { .. } => {
                            let _ = win.as_ref().window().move_window(Position::TrayCenter);
                            win.show().unwrap();
                            win.set_always_on_top(true).unwrap();
                        }
                        TrayIconEvent::Leave { .. } => {
                            // win.hide().unwrap();
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
