#![allow(non_camel_case_types)]

use std::os::raw::c_void;
use std::sync::mpsc;

use dispatch::{Queue, QueuePriority};
use objc2_core_location::CLLocationManager;
use objc2_core_wlan::CWWiFiClient;
use objc2_foundation::NSString;
use tracing::debug;

extern "C" {
    fn nw_retain(obj: *mut c_void) -> *mut c_void;
    fn nw_release(obj: *mut c_void) -> *mut c_void;
}

mod nw_interface;
mod nw_path;
mod nw_path_monitor;

use super::Monitor;
pub use nw_interface::*;
pub use nw_path::*;
pub use nw_path_monitor::*;

impl Monitor {
    pub fn new() -> Self {
        Self {}
    }
    pub fn start(&mut self) {
        let mut monitor = NWPathMonitor::create();

        let (tx, rx) = mpsc::channel();
        monitor.set_update_handler(move |path| {
            debug!(?path, "received path");
            tx.send(path).unwrap();
        });

        monitor.set_queue(Queue::global(QueuePriority::Low));

        monitor.start();

        for mut path in rx {
            if path.uses_wifi() {
                debug!("wifi connected");
            } else if path.uses_cellular() {
                debug!("cellular connected");
            } else if path.uses_wired() {
                debug!("wired connected");
            } else {
                debug!("unknown connection");
            }
            let (tx, rx) = mpsc::channel();

            path.enumerate_interfaces(move |interface| {
                debug!(?interface, "interface");
                tx.send(interface).unwrap();
                true
            });

            // tx will be dropped after enumerate_interfaces done, so rx is safe to iterate
            for mut interface in rx {
                if interface.is_wifi() {
                    let name = interface.get_name();
                    debug!(?name, "wifi interface");

                    unsafe {
                        let manager = CLLocationManager::new();
                        manager.startUpdatingLocation();
                        let status = manager.authorizationStatus();
                        debug!(?status, "location status");

                        let cli = CWWiFiClient::sharedWiFiClient();
                        if let Some(interface) =
                            cli.interfaceWithName(Some(&NSString::from_str(&name)))
                        {
                            let ssid = interface.ssid();
                            println!("ssid: {:?}", ssid);
                            debug!(?ssid, "ssid");
                        } else {
                            debug!("failed to get interface");
                        }
                    }
                    return;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitoring() {
        let mut m = Monitor::new();
        m.start();
    }
}
