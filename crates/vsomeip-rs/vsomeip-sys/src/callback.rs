use autocxx::c_void;
use cxx::SharedPtr;
use crate::ffi::vsomeip_v3::message;
use crate::ffi::vsomeip_v3::state_type_e;
use crate::instance_t;
use crate::service_t;

// pub struct CallbackRegistry<Key> {
//     map: HashMap<Key, *mut c_void>
// }
pub struct CallbackWrapper<CFun: Copy> {
    pub f: CFun,
    pub user_data: *mut std::ffi::c_void
}

impl<CFun: Copy> Drop for CallbackWrapper<CFun> {
    fn drop(&mut self) {
        // let _ = unsafe { Box::from_raw(self.user_data) };
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MessageHandlerCallback(
    pub unsafe extern "C" fn(message: *const SharedPtr<message>, user_data: *mut c_void),
);

impl MessageHandlerCallback {
    pub fn from_closure<F: FnMut(SharedPtr<message>) + 'static>(f: F) -> CallbackWrapper<Self> {
        let boxed = Box::new(f);
        let user_data = Box::into_raw(boxed) as *mut std::ffi::c_void;
        
        unsafe extern "C" fn call_closure<F>(message_ptr: *const SharedPtr<message>, user_data: *mut c_void) where F: FnMut(SharedPtr<message>) + 'static {
            let cb = user_data as *mut F;
            let message = unsafe { &*message_ptr };
            let message = message.clone();
            (*cb)(message);
        }

       CallbackWrapper {
        f: Self (call_closure::<F>),
        user_data
       }
    }
}

unsafe impl cxx::ExternType for MessageHandlerCallback {
    type Id = cxx::type_id!("message_handler_callback_t");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct StateHandlerCallback(
    pub unsafe extern "C" fn(state_type: state_type_e, user_data: *mut c_void),
);

impl StateHandlerCallback {
    pub fn from_closure<F: FnMut(state_type_e) + 'static>(f: F) -> CallbackWrapper<Self> {
        let boxed = Box::new(f);
        let user_data = Box::into_raw(boxed) as *mut std::ffi::c_void;
        
        unsafe extern "C" fn call_closure<F>(state_type: state_type_e, user_data: *mut c_void) where F: FnMut(state_type_e) + 'static {
            let cb = user_data as *mut F;
            (*cb)(state_type);
        }

       CallbackWrapper {
        f: Self (call_closure::<F>),
        user_data
       }
    }
}

unsafe impl cxx::ExternType for StateHandlerCallback {
    type Id = cxx::type_id!("state_handler_callback_t");
    type Kind = cxx::kind::Trivial;
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct AvailabilityHandlerCallback(
    pub unsafe extern "C" fn(service: u16, instance: u16, is_available: bool, user_data: *mut c_void),
);

impl AvailabilityHandlerCallback {
    pub fn from_closure<F: FnMut(service_t, instance_t, bool) + 'static>(f: F) -> CallbackWrapper<Self> {
        let boxed = Box::new(f);
        let user_data = Box::into_raw(boxed) as *mut std::ffi::c_void;
        
        unsafe extern "C" fn call_closure<F>(service: u16, instance: u16, is_available: bool, user_data: *mut c_void) where F: FnMut(service_t, instance_t, bool) + 'static {
            let cb = user_data as *mut F;
            (*cb)(service, instance, is_available);
        }

       CallbackWrapper {
        f: Self (call_closure::<F>),
        user_data
       }
    }
}

unsafe impl cxx::ExternType for AvailabilityHandlerCallback {
    type Id = cxx::type_id!("availability_handler_callback_t");
    type Kind = cxx::kind::Trivial;
}