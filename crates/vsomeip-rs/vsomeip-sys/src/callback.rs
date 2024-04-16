use autocxx::c_void;
use cxx::SharedPtr;
use crate::ffi::vsomeip_v3::message;

// pub struct CallbackRegistry<Key> {
//     map: HashMap<Key, *mut c_void>
// }
pub struct CallbackWrapper<CFun: Copy> {
    pub f: CFun,
    pub user_data: *mut std::ffi::c_void
}

impl<CFun: Copy> Drop for CallbackWrapper<CFun> {
    fn drop(&mut self) {
        let _ = unsafe { Box::from_raw(self.user_data) };
    }
}

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct MessageHandlerCallback(
    pub unsafe extern "C" fn(arg1: cxx::SharedPtr<message>, arg2: *mut c_void),
);

impl MessageHandlerCallback {
    pub fn from_closure<F: FnMut(SharedPtr<message>)>(f: F) -> CallbackWrapper<Self> {
        let boxed = Box::new(f);
        let user_data = Box::into_raw(boxed) as *mut std::ffi::c_void;
        
        unsafe extern "C" fn call_closure<F>(arg: SharedPtr<message>, user_data: *mut c_void) where F: FnMut(SharedPtr<message>) {
            let cb: &mut F = user_data.cast::<F>().as_mut().unwrap();
            (*cb)(arg);
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