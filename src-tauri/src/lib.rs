use log::{debug, error};
use smart_hosts::monitor::Monitor;
use smart_hosts_bridge::NetworkEvent;
use tauri::{
    tray::{TrayIconBuilder, TrayIconEvent},
    webview::WebviewWindowBuilder,
    AppHandle, Manager, WebviewUrl,
};
use tauri_utils::config::WindowEffectsConfig;

enum Window {
    Tray,
    Preferences,
}

impl Window {
    fn as_str(&self) -> &'static str {
        use Window::*;
        match self {
            Tray => "Tray",
            Preferences => "Preferences",
        }
    }
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn open_preferences(app: AppHandle) {
    if let Some(win) = app.get_window(Window::Preferences.as_str()) {
        win.show().unwrap();
    } else {
        error!("preferences window not found");
    }
}

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
    let app = tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![monitor, open_preferences])
        .setup(|app| {
            let monitor = Monitor::default();
            monitor.start();
            app.manage(monitor);

            WebviewWindowBuilder::new(app, Window::Preferences.as_str(), WebviewUrl::default())
                .title("Smart Hosts - Preferences")
                .visible(false)
                .decorations(false)
                .skip_taskbar(true) // not working in MacOS
                .transparent(true)
                .shadow(true)
                .effects(WindowEffectsConfig {
                    radius: Some(12.0),
                    ..Default::default()
                })
                .inner_size(800.0, 600.0)
                .resizable(false)
                .build()?;

            WebviewWindowBuilder::new(app, Window::Tray.as_str(), WebviewUrl::default())
                .title("Smart Hosts - Tray")
                .visible(false)
                .decorations(false)
                .skip_taskbar(true) // not working in MacOS
                .transparent(true)
                .shadow(true)
                .effects(WindowEffectsConfig {
                    radius: Some(12.0),
                    ..Default::default()
                })
                .inner_size(300.0, 200.0)
                .resizable(false)
                .always_on_top(true)
                .build()?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu_on_left_click(true)
                .on_tray_icon_event(|tray_handle, event| {
                    use tauri_plugin_positioner::{Position, WindowExt};

                    let app = tray_handle.app_handle();
                    tauri_plugin_positioner::on_tray_event(app, &event);

                    let win = app.get_window(Window::Tray.as_str()).unwrap();
                    match event {
                        TrayIconEvent::Click { .. } => {
                            win.move_window(Position::TrayCenter).unwrap();
                            win.set_focus().unwrap();
                            win.show().unwrap();
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
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    app.run(|app, event| match event {
        tauri::RunEvent::ExitRequested { api, .. } => {
            debug!("exit requested");
            api.prevent_exit();
        }
        tauri::RunEvent::WindowEvent { event, label, .. } => {
            debug!("window event from {:?}: {:?}", label, event);
            use tauri::WindowEvent::*;
            match event {
                CloseRequested { api, .. } => {
                    api.prevent_close();
                    let win = app.get_window(&label).unwrap();
                    win.hide().unwrap();
                }
                Focused(focused) if !focused && label == Window::Tray.as_str() => {
                    let win = app.get_window(&label).unwrap();
                    win.hide().unwrap();
                }
                _ => {}
            }
        }
        _ => {}
    })
}
