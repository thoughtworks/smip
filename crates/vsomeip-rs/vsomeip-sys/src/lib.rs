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
    generate!("vsomeip_v3::payload")
    generate!("vsomeip_v3::state_type_e")
    // What types and functions we want to generate
    // generate_pod!("application")
    // generate!("print_point")
}

#[cxx::bridge]
mod ffi2 {

    extern "C++" {
        include!("shim.hpp");
        #[namespace = "vsomeip_v3"]
        type application = crate::ffi::vsomeip_v3::application;
        type message_handler_callback_t = crate::callback::MessageHandlerCallback;
        type state_handler_callback_t = crate::callback::StateHandlerCallback;
        type c_void;

        unsafe fn application_register_message_handler(application: Pin<&mut application>, _service: u16, _instance: u16, _method: u16, _handler: message_handler_callback_t, user_data: *mut c_void);
        unsafe fn application_register_state_handler(application: Pin<&mut application>, _handler: state_handler_callback_t, user_data: *mut c_void);
    }
}

pub use ffi::vsomeip_v3::*;

pub use callback::*;
pub use ffi2::application_register_message_handler;
pub use ffi2::application_register_state_handler;
