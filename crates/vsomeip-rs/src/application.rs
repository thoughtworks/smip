use cxx::*;

use crate::{primitives::{InstanceId, MajorVersion, MethodId, MinorVersion, ServiceId}, util::AsPinMut, Message};

pub struct Application {
    pub(crate) inner: SharedPtr<vsomeip_sys::application>,
}

unsafe impl AsPinMut for Application {
    type Inner = vsomeip_sys::application;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}

impl Application {
    pub fn name(&self) -> &str {
        let name_cxx = self.inner.get_name();

        name_cxx.to_str().unwrap()
    }
    pub(crate) fn init(&self) -> bool {
        vsomeip_sys::application::init(self.pin_mut())
    }
    pub fn start(&self) {
        vsomeip_sys::application::start(self.pin_mut())
    }
    pub fn stop(&self) {
        vsomeip_sys::application::stop(self.pin_mut())
    }
    pub fn offer_service(&self, service_id: ServiceId, instance_id: InstanceId, major_version: MajorVersion, minor_version: MinorVersion) {
        vsomeip_sys::application::offer_service(self.pin_mut(), service_id, instance_id, major_version, minor_version);
    }
    pub fn stop_offer_service(&self, service_id: ServiceId, instance_id: InstanceId, major_version: MajorVersion, minor_version: MinorVersion) {
        vsomeip_sys::application::stop_offer_service(self.pin_mut(), service_id, instance_id, major_version, minor_version);
    }
    pub fn send(&self, message: &Message) {
        vsomeip_sys::application::send(self.pin_mut(), message.inner.clone());
    }
    pub fn register_message_handler<F: FnMut(&Message)>(&self, service_id: ServiceId, instance_id: InstanceId, method_id: MethodId, mut f: F) {
        
        let message_callback = vsomeip_sys::MessageHandlerCallback::from_closure(|raw_message| {
            let message = Message {
                inner: raw_message
            };

            (f)(&message);
        });


        unsafe { 
            vsomeip_sys::application_register_message_handler(self.pin_mut(), service_id, instance_id, method_id, message_callback.f, message_callback.user_data as *mut _); 
        }

    }
}