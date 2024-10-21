#![allow(dead_code)]
#![allow(unused_imports)]

#[derive(Debug, Default, Clone, PartialEq)]
pub enum NetworkInfo {
    WiFi {
        ssid: String,
        interface: String,
    },
    Cellular {
        interface: String,
    },
    Wired {
        interface: String,
    },
    #[default]
    Unknown,
}

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub use macos::Monitor;
