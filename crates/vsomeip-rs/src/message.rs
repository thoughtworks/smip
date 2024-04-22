use cxx::*;

use crate::{util::AsPinMut, Payload, Runtime};
pub struct Message {
    pub(crate) inner: SharedPtr<vsomeip_sys::message>
}

unsafe impl AsPinMut for Message {
    type Inner = vsomeip_sys::message;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}

impl Message {
    pub fn new(reliable: bool) -> Self {
        Runtime::get().create_message(reliable)
    }
    pub fn request(reliable: bool) -> Message {
        Runtime::get().create_request(reliable)
    }
    pub fn response(request: &Message) -> Message {
        Runtime::get().create_response(request) 
    } 
    pub fn get_payload(&self) -> Payload {
        let payload = unsafe { self.inner.get_payload() };

        Payload {
            inner: payload
        }
    }
    pub fn set_payload(&mut self, payload: &Payload) {
        unsafe { vsomeip_sys::message::set_payload(self.pin_mut(), payload.inner.clone()) };
    }
}