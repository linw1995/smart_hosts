use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type Window;

    #[wasm_bindgen(method, getter = label)]
    pub fn get_label(this: &Window) -> String;

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
