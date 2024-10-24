use leptos::WriteSignal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Window;

    #[wasm_bindgen(method, getter = label)]
    pub fn get_label(this: &Window) -> String;

    #[wasm_bindgen(method)]
    pub fn close(this: &Window);

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "window"], js_name = getCurrentWindow)]
    pub fn get_current_window() -> Window;

    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    pub async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "log"], js_name = debug)]
    pub async fn send_debug(msg: String);
}

pub mod log {
    macro_rules! debug {
        ($template:literal, $expresion:expr) => {{
            use crate::utils::send_debug;
            send_debug(format!($template, $expresion))
        }};
    }
    pub(crate) use debug;
}

#[derive(Clone)]
pub struct ThemeCtx(pub WriteSignal<String>);

pub fn load_theme() -> Option<String> {
    let window = web_sys::window()?;
    let storage = window.local_storage().unwrap_or(None)?;
    storage.get_item("theme").unwrap_or(None)
}

pub fn save_theme(theme: &str) {
    let window = web_sys::window().unwrap();
    let storage = window.local_storage().unwrap().unwrap();
    storage.set_item("theme", theme).unwrap();
}

pub fn toggle_theme(theme: &mut String){
    *theme = if theme == "light" {
        "night".to_string()
    } else {
        "light".to_string()
    }
}
