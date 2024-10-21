#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use std::os::raw::c_void;

use block2::{Block, StackBlock};
use objc2::runtime::{Bool, ProtocolObject};
use objc2::{extern_protocol, ProtocolType};
use objc2_foundation::NSObjectProtocol;

use super::{nw_interface_t, nw_release, nw_retain, NWInterface, NWInterfaceType};

extern_protocol!(
    pub(crate) unsafe trait OS_nw_path: NSObjectProtocol {}
    unsafe impl ProtocolType for dyn OS_nw_path {}
);
pub(crate) type nw_path_t = ProtocolObject<dyn OS_nw_path>;
pub(crate) type nw_path_enumerate_interfaces_block_t = Block<dyn Fn(*mut nw_interface_t) -> Bool>;

extern "C" {
    fn nw_path_uses_interface_type(path: *mut nw_path_t, interface_type: NWInterfaceType) -> bool;
    fn nw_path_enumerate_interfaces(
        path: *mut nw_path_t,
        enumerate_block: &nw_path_enumerate_interfaces_block_t,
    ) -> c_void;
}

#[derive(Debug)]
pub struct NWPath {
    raw: *mut nw_path_t,
}

impl NWPath {
    pub(crate) fn new(raw: *mut nw_path_t) -> Self {
        unsafe {
            // Retain the raw pointer to avoid it being deallocated automatically
            nw_retain(raw.cast())
        };
        Self { raw }
    }

    pub fn uses(&mut self, interface_type: NWInterfaceType) -> bool {
        unsafe { nw_path_uses_interface_type(self.raw, interface_type) }
    }

    pub fn uses_wifi(&mut self) -> bool {
        self.uses(NWInterfaceType::WIFI)
    }

    pub fn uses_cellular(&mut self) -> bool {
        self.uses(NWInterfaceType::CELLULAR)
    }

    pub fn uses_wired(&mut self) -> bool {
        self.uses(NWInterfaceType::WIRED)
    }

    pub fn enumerate_interfaces(
        &mut self,
        enumerate_block: impl Fn(NWInterface) -> bool + std::clone::Clone + 'static,
    ) {
        unsafe {
            let block = StackBlock::new(move |i| {
                let i = NWInterface::new(i);
                Bool::from(enumerate_block(i))
            });
            nw_path_enumerate_interfaces(self.raw, &block);
        }
    }
}

impl Drop for NWPath {
    fn drop(&mut self) {
        unsafe {
            nw_release(self.raw.cast());
        }
    }
}
