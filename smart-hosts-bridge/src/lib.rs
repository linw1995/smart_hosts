use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum NetworkEvent {
    WiFi { ssid: String, interface: String },
    Wired { interface: String },
    Cellular { interface: String },
    Unknown,
}
