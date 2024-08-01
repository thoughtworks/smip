#![allow(unused_imports)]
#![allow(clippy::missing_safety_doc)]

use autocxx::prelude::*;
#[allow(unused)]
use cxx::*;

mod callback;

include_cpp! {
    // C++ headers we want to include.
    #include "vsomeip/vsomeip.hpp"
    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    generate!("vsomeip_v3::runtime")
    generate!("vsomeip_v3::application")
    generate!("vsomeip_v3::message")
    generate!("vsomeip_v3::message_base")
    generate!("vsomeip_v3::payload")
    generate!("vsomeip_v3::state_type_e")
}

#[cxx::bridge]
mod ffi2 {
    extern "C++" {
        include!("shim.hpp");
        #[namespace = "vsomeip_v3"]
        type message = crate::ffi::vsomeip_v3::message;
        #[namespace = "vsomeip_v3"]
        type message_base = crate::ffi::vsomeip_v3::message_base;
        unsafe fn as_message_base(message: &SharedPtr<message>) -> SharedPtr<message_base>;
    }
    extern "C++" {
        include!("shim.hpp");
        #[namespace = "vsomeip_v3"]
        type application = crate::ffi::vsomeip_v3::application;
        type message_handler_callback_t = crate::callback::MessageHandlerCallback;
        type state_handler_callback_t = crate::callback::StateHandlerCallback;
        type availability_handler_callback_t = crate::callback::AvailabilityHandlerCallback;
        type c_void;

        unsafe fn application_register_message_handler(application: Pin<&mut application>, _service: u16, _instance: u16, _method: u16, _handler: message_handler_callback_t, user_data: *mut c_void);
        unsafe fn application_register_state_handler(application: Pin<&mut application>, _handler: state_handler_callback_t, user_data: *mut c_void);
        unsafe fn application_register_availability_handler(application: Pin<&mut application>, _service: u16, _instance: u16, _handler: availability_handler_callback_t, _major: u8, _minor: u32, user_data: *mut c_void);
    }
}

pub use ffi::vsomeip_v3::*;

pub use callback::*;
pub use ffi2::application_register_message_handler;
pub use ffi2::application_register_state_handler;
pub use ffi2::application_register_availability_handler;
pub use ffi2::as_message_base;