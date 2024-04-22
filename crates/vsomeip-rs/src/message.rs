use cxx::*;

use crate::{util::{self, AsPinMut}, InstanceId, MessageType, MethodId, Payload, ReturnCode, Runtime, ServiceId};
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
    pub fn get_service(&self) -> ServiceId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_service(&message_base) }
    }
    pub fn get_instance(&self) -> InstanceId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_instance(&message_base) }
    }
    pub fn get_method(&self) -> MethodId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_method(&message_base) }
    }
    pub fn get_return_code(&self) -> ReturnCode {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let return_code = unsafe { vsomeip_sys::message_base::get_return_code(&message_base) };
        return_code.into()
    }
    pub fn get_message_type(&self) -> MessageType {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let message_type = unsafe { vsomeip_sys::message_base::get_message_type(&message_base) };
        message_type.into()
    }
    pub fn get_payload(&self) -> Payload {
        let payload = unsafe { self.inner.get_payload() };

        Payload {
            inner: payload
        }
    }
    pub fn set_service(&mut self, service_id: u16) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_service(pin_mut, service_id) };
    }
    pub fn set_instance(&mut self, instance_id: u16) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_instance(pin_mut, instance_id) };
    }
    pub fn set_method(&mut self, method_id: u16) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_method(pin_mut, method_id) };
    }
    pub fn set_return_code(&mut self, return_code: ReturnCode) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_return_code(pin_mut, return_code.into()) };
    }
    pub fn set_payload(&mut self, payload: &Payload) {
        unsafe { vsomeip_sys::message::set_payload(self.pin_mut(), payload.inner.clone()) };
    }
    
}