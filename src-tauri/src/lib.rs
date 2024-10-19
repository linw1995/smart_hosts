use smart_hosts_bridge::NetworkEvent;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn monitor() -> NetworkEvent {
    use NetworkEvent::*;

    Unknown
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![monitor])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
