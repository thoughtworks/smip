use autocxx::prelude::*;
use cxx::*;

include_cpp! {
    // C++ headers we want to include.
    #include "vsomeip/vsomeip.hpp"
    // Safety policy. We are marking that this whole C++ inclusion is unsafe
    // which means the functions themselves do not need to be marked
    // as unsafe. Other policies are possible.
    safety!(unsafe)
    generate!("vsomeip_v3::runtime")
    generate!("vsomeip_v3::application")
    generate!("vsomeip_v3::message")
    generate!("vsomeip_v3::payload")
    // What types and functions we want to generate
    // generate_pod!("application")
    // generate!("print_point")
}

#[cxx::bridge[namespace = "vsomeip_v3"]]
mod ffi2 {
    unsafe extern "C++" {
        include!("shim.hpp");
        type application = ffi::vsomeip_v3::application;
        type message = ffi::vsomeip_v3::message;
        type c_void;
        type shim_message_handler_t = extern "C" fn(arg1: SharedPtr<message>, arg2: *mut c_void);

        unsafe fn application_register_message_handler(application: &application, _service: u16, _instance: u16, _method: u16, _handler: shim_message_handler_t, user_data: *mut c_void);
    }
}

pub use ffi::vsomeip_v3::*;