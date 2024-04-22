use cxx::*;

use crate::{util::AsPinMut, Application, Message, Payload};

pub struct Runtime {
    inner: SharedPtr<vsomeip_sys::runtime>
}

unsafe impl AsPinMut for Runtime {
    type Inner = vsomeip_sys::runtime;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}
impl Runtime {
    pub fn get() -> Self {
        Self {
            inner: unsafe  { vsomeip_sys::runtime::get() }
        }
    }
    pub fn create_application_with_name(&self, name: impl AsRef<str>) -> Application {
        let name = name.as_ref();
        let_cxx_string!(name_cxx = name);
        let application = unsafe { vsomeip_sys::runtime::create_application(self.pin_mut(), &name_cxx) }; 

        let app = Application {
            inner: application,
        };

        if !app.init() {
            panic!("Failed to init application");
        }

        app
    }
    pub fn create_payload(&self) -> Payload {
        let inner = unsafe { self.inner.create_payload() };

        Payload {
            inner
        }
    }
    pub fn create_message(&self, reliable: bool) -> Message {
        let inner = unsafe { self.inner.create_message(reliable) };

        Message {
            inner
        }
    }
    pub fn create_response(&self, request: &Message) -> Message {
        let raw_message = unsafe {vsomeip_sys::runtime::create_response(&self.inner, request.inner())};

        Message {
            inner: raw_message
        }
    }
    pub fn create_request(&self, reliable: bool) -> Message {
        let raw_message = unsafe { vsomeip_sys::runtime::create_request(&self.inner, reliable) };

        Message {
            inner: raw_message
        }
    }
}