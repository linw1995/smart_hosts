#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use std::os::raw::c_void;

use block2::{Block, RcBlock};
use dispatch::{ffi::dispatch_queue_t, Queue};
use objc2::runtime::ProtocolObject;
use objc2::{extern_protocol, ProtocolType};
use objc2_foundation::NSObjectProtocol;

use super::{nw_path_t, nw_release, nw_retain, NWPath};

extern_protocol!(
    pub(crate) unsafe trait OS_nw_path_monitor: NSObjectProtocol {}
    unsafe impl ProtocolType for dyn OS_nw_path_monitor {}
);
pub(crate) type nw_path_monitor_t = ProtocolObject<dyn OS_nw_path_monitor>;
pub(crate) type nw_path_monitor_update_handler_t = Block<dyn Fn(*mut nw_path_t)>;

extern "C" {
    fn nw_path_monitor_create() -> *mut nw_path_monitor_t;
    fn nw_path_monitor_set_queue(
        monitor: *mut nw_path_monitor_t,
        queue: dispatch_queue_t,
    ) -> c_void;
    fn nw_path_monitor_start(monitor: *mut nw_path_monitor_t) -> c_void;
    fn nw_path_monitor_set_update_handler(
        monitor: *mut nw_path_monitor_t,
        handler: &nw_path_monitor_update_handler_t,
    ) -> c_void;
}

#[derive(Debug)]
pub struct NWPathMonitor {
    raw: *mut nw_path_monitor_t,
}

impl NWPathMonitor {
    pub fn create() -> Self {
        unsafe {
            let ptr = nw_path_monitor_create();
            // Retain the raw pointer to avoid it being deallocated automatically
            nw_retain(ptr.cast());
            Self { raw: ptr }
        }
    }
    pub fn set_queue(&mut self, queue: Queue) {
        unsafe {
            nw_path_monitor_set_queue(self.raw, queue.as_raw());
        }
    }
    pub fn set_update_handler(&mut self, handler: impl Fn(NWPath) + std::clone::Clone + 'static) {
        unsafe {
            nw_path_monitor_set_update_handler(
                self.raw,
                &RcBlock::new(move |p: *mut nw_path_t| {
                    let p = NWPath::new(p);
                    handler(p);
                }),
            );
        }
    }
    pub fn start(&mut self) {
        unsafe {
            nw_path_monitor_start(self.raw);
        }
    }
}

impl Drop for NWPathMonitor {
    fn drop(&mut self) {
        unsafe {
            nw_release(self.raw.cast());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use dispatch::QueuePriority;
    use log::debug;

    #[test]
    fn monitor_network() {
        let mut monitor = NWPathMonitor::create();
        debug!("monitor {monitor:?} created");

        monitor.set_update_handler(|mut p| {
            if p.uses_cellular() {
                debug!("connection using cellular");
            } else if p.uses_wifi() {
                debug!("connection using wifi");
            } else {
                debug!("connection without using wifi or cellular");
                return;
            }

            p.enumerate_interfaces(|mut i| {
                let name = i.get_name();
                let typ = i.get_type();
                debug!("enumerating interface {name:?} type {typ:?}");
                true
            });
        });
        debug!("set update handler success");

        let queue = Queue::global(QueuePriority::Low);
        debug!("queue global created: {queue:?}");
        monitor.set_queue(queue);
        debug!("set queue success");

        monitor.start();
        debug!("start monitor success");

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
