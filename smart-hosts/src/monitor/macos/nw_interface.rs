#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]

use std::os::raw::c_char;

use objc2::runtime::ProtocolObject;
use objc2::{extern_protocol, Encode, Encoding, ProtocolType, RefEncode};
use objc2_foundation::{NSObjectProtocol, NSUInteger};

use super::{nw_release, nw_retain};

extern_protocol!(
    pub(crate) unsafe trait OS_nw_interface: NSObjectProtocol {}
    unsafe impl ProtocolType for dyn OS_nw_interface {}
);
pub(crate) type nw_interface_t = ProtocolObject<dyn OS_nw_interface>;

extern "C" {
    fn nw_interface_get_type(interface: *mut nw_interface_t) -> NWInterfaceType;
    fn nw_interface_get_name(interface: *mut nw_interface_t) -> *const c_char;
}

#[derive(Debug)]
pub struct NWInterface {
    raw: *mut nw_interface_t,
}

impl NWInterface {
    pub(crate) fn new(raw: *mut nw_interface_t) -> Self {
        unsafe {
            // Retain the raw pointer to avoid it being deallocated automatically
            nw_retain(raw.cast());
        }
        Self { raw }
    }
    pub fn get_type(&mut self) -> NWInterfaceType {
        unsafe { nw_interface_get_type(self.raw) }
    }
    pub fn get_name(&mut self) -> String {
        unsafe {
            let cstr = nw_interface_get_name(self.raw);
            let cstr = std::ffi::CStr::from_ptr(cstr);
            cstr.to_string_lossy().into_owned()
        }
    }

    pub fn is_wifi(&mut self) -> bool {
        self.get_type() == NWInterfaceType::WIFI
    }

    pub fn is_cellular(&mut self) -> bool {
        self.get_type() == NWInterfaceType::CELLULAR
    }

    pub fn is_wired(&mut self) -> bool {
        self.get_type() == NWInterfaceType::WIRED
    }
}

impl Drop for NWInterface {
    fn drop(&mut self) {
        unsafe {
            nw_release(self.raw.cast());
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NWInterfaceType(pub NSUInteger);
#[allow(dead_code)]
impl NWInterfaceType {
    pub const OTHER: Self = Self(0);
    pub const WIFI: Self = Self(1);
    pub const CELLULAR: Self = Self(2);
    pub const WIRED: Self = Self(3);
    pub const LOOPBACK: Self = Self(4);
}

unsafe impl Encode for NWInterfaceType {
    const ENCODING: Encoding = NSUInteger::ENCODING;
}

unsafe impl RefEncode for NWInterfaceType {
    const ENCODING_REF: Encoding = Encoding::Pointer(&Self::ENCODING);
}

impl std::fmt::Display for NWInterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            0 => write!(f, "OTHER"),
            1 => write!(f, "WIFI"),
            2 => write!(f, "CELLULAR"),
            3 => write!(f, "WIRED"),
            4 => write!(f, "LOOPBACK"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}
