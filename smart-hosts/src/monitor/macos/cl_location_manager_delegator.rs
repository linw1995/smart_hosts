#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::sync::mpsc::Sender;

use block2::RcBlock;
use log::debug;
use objc2::encode::OptionEncode;
use objc2::rc::{Allocated, Retained};
use objc2::runtime::NSObject;
use objc2::{
    declare_class, msg_send, msg_send_id, mutability, ClassType, DeclaredClass, Encode, Encoding,
    RefEncode,
};
use objc2_core_location::{CLAuthorizationStatus, CLLocationManager, CLLocationManagerDelegate};
use objc2_foundation::NSObjectProtocol;

#[derive(Debug, Clone)]
pub struct EncodedSender<T = CLAuthorizationStatus>(Sender<T>);

unsafe impl<T> Encode for EncodedSender<T> {
    const ENCODING: Encoding = Encoding::Object;
}

unsafe impl<T> OptionEncode for EncodedSender<T> {}

#[derive(Clone)]
pub struct Ivars {
    sender: Option<EncodedSender>,
}

declare_class!(
    #[derive(Debug)]
    pub(crate) struct LocationManagerDelegator;

    unsafe impl ClassType for LocationManagerDelegator {
        type Super = NSObject;
        type Mutability = mutability::Mutable;
        const NAME: &'static str = "LocationManagerDelegator";
    }

    impl DeclaredClass for LocationManagerDelegator {
        type Ivars = Ivars;
    }

    unsafe impl LocationManagerDelegator {
        #[method_id(initWithSender:)]
        fn init(this: Allocated<Self>, sender: Option<EncodedSender>) -> Option<Retained<Self>> {
            let this = this.set_ivars(Ivars {
                sender,
            });
            unsafe { msg_send_id![super(this), init] }
        }
    }
    unsafe impl NSObjectProtocol for LocationManagerDelegator {}

    unsafe impl CLLocationManagerDelegate for LocationManagerDelegator {
        #[method(locationManagerDidChangeAuthorization:)]
        unsafe fn locationManagerDidChangeAuthorization(&self, manager: &CLLocationManager) {
            if let Some(sender) = self.ivars().sender.clone() {
                let status = manager.authorizationStatus();
                if matches!(status, CLAuthorizationStatus::kCLAuthorizationStatusNotDetermined){
                    debug!("request authorization");
                    // manager.requestAlwaysAuthorization();
                    manager.requestWhenInUseAuthorization();
                }
                debug!("sending authorization status: {:?}", status);
                sender.0.send(status).unwrap();
            } else {
            debug!("locationManagerDidChangeAuthorization");
        }
        }
    }
);

impl LocationManagerDelegator {
    pub fn new(sender: Option<Sender<CLAuthorizationStatus>>) -> Retained<Self> {
        unsafe {
            let sender = sender.map(EncodedSender);
            msg_send_id![Self::alloc(), initWithSender: sender]
        }
    }
}
