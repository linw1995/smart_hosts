#![allow(non_camel_case_types)]

use std::os::raw::c_void;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};

use block2::StackBlock;
use dispatch::{Queue, QueuePriority};
use log::debug;
use objc2::declare_class;
use objc2::rc::Retained;
use objc2::runtime::ProtocolObject;
use objc2_core_location::{CLAuthorizationStatus, CLLocationManager, CLLocationManagerDelegate};
use objc2_core_wlan::CWWiFiClient;
use objc2_foundation::{NSDefaultRunLoopMode, NSRunLoop, NSRunLoopCommonModes, NSString, NSTimer};

extern "C" {
    fn nw_retain(obj: *mut c_void) -> *mut c_void;
    fn nw_release(obj: *mut c_void) -> *mut c_void;
}

mod cl_location_manager_delegator;
mod nw_interface;
mod nw_path;
mod nw_path_monitor;

use super::NetworkInfo;
pub use cl_location_manager_delegator::*;
pub use nw_interface::*;
pub use nw_path::*;
pub use nw_path_monitor::*;

#[derive(Default)]
pub struct Monitor {
    permission_granted: Arc<Mutex<bool>>,
    info: Arc<Mutex<NetworkInfo>>,
}

fn get_wifi_ssid(name: &str) -> String {
    unsafe {
        let cli = CWWiFiClient::sharedWiFiClient();
        let name = NSString::from_str(name);
        if let Some(interface) = cli.interfaceWithName(Some(&name)) {
            let ssid = interface.ssid().map(|s| s.to_string()).unwrap_or_default();
            debug!("the ssid of WiFi interface {name:?}: {ssid:?}");
            ssid
        } else {
            debug!("failed to get ssid of interface: {name:?}");
            String::new()
        }
    }
}

impl Monitor {
    pub fn start(&self) {
        let (authorization_status_tx, authorization_status_rx) = mpsc::channel();

        std::thread::spawn(move || unsafe {
            let manager = CLLocationManager::new();

            let delegator = LocationManagerDelegator::new(Some(authorization_status_tx));
            let delegator = ProtocolObject::from_retained(delegator);
            manager.setDelegate(Some(&delegator));

            // A run loop is required for the location manager to work when it's not running on the main thread.
            let run_loop = NSRunLoop::currentRunLoop();
            // Run lop needs some task to schedule.
            // Otherwise, the run loop will exit immediately.
            let timer = NSTimer::scheduledTimerWithTimeInterval_repeats_block(
                60.0,
                true,
                &StackBlock::new(|_t| {}),
            );
            run_loop.addTimer_forMode(&timer, NSDefaultRunLoopMode);
            run_loop.run();
            debug!("exiting location manager thread");
        });

        let info = self.info.clone();
        let mut monitor = NWPathMonitor::create();
        monitor.set_update_handler(move |mut path| {
            debug!("received path: {path:?}");
            let status = path.get_status();
            debug!("received path status: {status}");

            if path.uses_wifi() {
                debug!("wifi connected");
            } else if path.uses_cellular() {
                debug!("cellular connected");
            } else if path.uses_wired() {
                debug!("wired connected");
            } else {
                debug!("unknown connection");
            }

            let info = info.clone();
            path.enumerate_interfaces(move |mut interface| {
                debug!("enumerating interface: {interface:?}");

                let name = interface.get_name();

                let event = if interface.is_wifi() {
                    debug!("wifi interface: {name:?}");

                    let ssid = get_wifi_ssid(&name);
                    NetworkInfo::WiFi {
                        ssid,
                        interface: name,
                    }
                } else if interface.is_cellular() {
                    debug!("cellular interface: {name:?}");
                    NetworkInfo::Cellular { interface: name }
                } else if interface.is_wired() {
                    debug!("wired interface: {name:?}");
                    NetworkInfo::Wired { interface: name }
                } else {
                    debug!("unknown interface");
                    NetworkInfo::Unknown
                };

                let mut val = info.lock().unwrap();
                *val = event;

                true
            });
        });
        monitor.set_queue(Queue::global(QueuePriority::Low));
        monitor.start();

        let info = self.info.clone();
        let granted = self.permission_granted.clone();
        std::thread::spawn(move || {
            for status in authorization_status_rx {
                debug!("current location authorizationStatus: {status:?}");
                let mut granted = granted.lock().unwrap();
                match status {
                    CLAuthorizationStatus::kCLAuthorizationStatusAuthorizedAlways
                    | CLAuthorizationStatus::kCLAuthorizationStatusAuthorizedWhenInUse => {
                        *granted = true;

                        let event = info.lock().unwrap().clone();
                        match event {
                            NetworkInfo::WiFi { ssid, interface } if ssid.is_empty() => {
                                debug!("updating WiFi ssid");

                                let ssid = get_wifi_ssid(&interface);
                                *info.lock().unwrap() = NetworkInfo::WiFi { ssid, interface };
                            }
                            _ => {}
                        }
                    }
                    _ => {
                        *granted = false;
                    }
                }
            }
            debug!("exiting authorization statue update thread");
        });
    }

    pub fn get_network_info(&self) -> NetworkInfo {
        self.info.lock().unwrap().clone()
    }

    pub fn is_permission_granted(&self) -> bool {
        *self.permission_granted.lock().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn monitoring() {
        let m = Monitor::default();
        m.start();

        std::thread::sleep(std::time::Duration::from_secs(10));

        let info = m.get_network_info();
        log::debug!("network info: {info:?}");
    }
}
