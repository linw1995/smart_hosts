#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use std::os::raw::c_void;

use block2::{Block, StackBlock};
use objc2::runtime::{Bool, ProtocolObject};
use objc2::{extern_protocol, Encode, Encoding, ProtocolType, RefEncode};
use objc2_foundation::{NSObjectProtocol, NSUInteger};

use super::{nw_interface_t, nw_release, nw_retain, NWInterface, NWInterfaceType};

extern_protocol!(
    pub(crate) unsafe trait OS_nw_path: NSObjectProtocol {}
    unsafe impl ProtocolType for dyn OS_nw_path {}
);
pub(crate) type nw_path_t = ProtocolObject<dyn OS_nw_path>;
pub(crate) type nw_path_enumerate_interfaces_block_t = Block<dyn Fn(*mut nw_interface_t) -> Bool>;

extern "C" {
    fn nw_path_get_status(path: *mut nw_path_t) -> NWPathStatus;
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

    pub fn get_status(&mut self) -> NWPathStatus {
        unsafe { nw_path_get_status(self.raw) }
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

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NWPathStatus(pub NSUInteger);
#[allow(dead_code)]
impl NWPathStatus {
    /// The path is not valid
    pub const INVALID: Self = Self(0);
    /// The path has a usable route upon which to send and receive data
    pub const SATISFIED: Self = Self(1);
    /// The path does not have a usable route. This may be due to a network interface being down, or due to system policy.
    pub const UNSATISFIED: Self = Self(2);
    /// The path does not currently have a usable route, but a connection attempt will trigger network attachment
    pub const SATISFIABLE: Self = Self(3);
}

unsafe impl Encode for NWPathStatus {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NWPathStatus {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl std::fmt::Display for NWPathStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "INVALID"),
            1 => write!(f, "SATISFIED"),
            2 => write!(f, "UNSATISFIED"),
            3 => write!(f, "SATIFIABLE"),
            _ => write!(f, "UNKNOWN({})", self.0),
        }
    }
}
