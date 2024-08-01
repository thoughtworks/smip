use cxx::*;

use crate::*;
use crate::util::AsPinMut;

/// Represents a SOME/IP Message.
/// 
/// Except SOME/IP Service Discovery messages, all SOME/IP messages within vsomeip
/// are represented by message objects.
pub struct Message {
    pub(crate) inner: SharedPtr<vsomeip_sys::message>
}

impl Clone for Message {
    fn clone(&self) -> Self {
        let mut cloned = Message::new(self.is_reliable());
        
        cloned.set_service(self.get_service());
        cloned.set_instance(self.get_instance());
        cloned.set_client(self.get_client());
        cloned.set_session(self.get_session());
        cloned.set_interface_version(self.get_interface_version());
        cloned.set_method(self.get_method());
        cloned.set_return_code(self.get_return_code());
        cloned.set_message_type(self.get_message_type());
        cloned.set_payload(&self.get_payload());

        cloned
    }

}

unsafe impl Send for Message {}
unsafe impl Sync for Message {}

unsafe impl AsPinMut for Message {
    type Inner = vsomeip_sys::message;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}

impl Message {
    /// Creates a new message object.
    /// Shorthand for [`Runtime::get().create_message(...)`](struct.Runtime.html#method.create_message)
    pub fn new(reliable: bool) -> Self {
        Runtime::get().create_message(reliable)
    }
    /// Creates a new request message object.
    /// Shorthand for [`Runtime::get().create_request(...)`](struct.Runtime.html#method.create_request)
    pub fn request(reliable: bool) -> Message {
        Runtime::get().create_request(reliable)
    }
    /// Creates a new response message object.
    /// Shorthand for [`Runtime::get().create_response(...)`](struct.Runtime.html#method.create_response)
    pub fn response(request: &Message) -> Message {
        Runtime::get().create_response(request) 
    } 

    /// Returns the service identifier from the message header.
    pub fn get_service(&self) -> ServiceId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_service(&message_base) }
    }

    /// Sets the service identifier in the message header.
    pub fn set_service(&mut self, service_id: ServiceId) {
        // FIXME: Converting to SharedPtr<message_base> is costly 
        // autocxx doesn't generate methods on the base class currently
        // Ideally we should have wrapper functions the methods on the base class message_base in C++
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_service(pin_mut, service_id) };
    }

    /// Returns the instance identifier from the message header.
    pub fn get_instance(&self) -> InstanceId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_instance(&message_base) }
    }

    /// Sets the instance identifier in the message header.
    pub fn set_instance(&mut self, instance_id: InstanceId) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_instance(pin_mut, instance_id) };
    }

    /// Gets the client identifier from the message header.
    pub fn get_client(&self) -> ClientId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_client(&message_base) }
    }

    /// Sets the client identifier in the message header.
    pub fn set_client(&mut self, client_id: ClientId) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_client(pin_mut, client_id) };
    }

    /// Gets the session identifier from the message header.
    pub fn get_session(&self) -> SessionId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_session(&message_base) }
    }

    /// Sets the session identifier in the message header.
    pub fn set_session(&mut self, session_id: SessionId) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_session(pin_mut, session_id) };
    }

    /// Get the request identifier from the message header.
    ///
    /// The request identifier consists of the client identifier and the
    /// session identifier. As it does really make sense to set it as
    /// a whole, setting is not supported.
    pub fn get_request(&self) -> RequestId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_request(&message_base) }
    }

    /// Gets the method/event identifier from the message header.
    pub fn get_method(&self) -> MethodId {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_method(&message_base) }
    }

    /// Sets the method/event identifier in the message header.
    pub fn set_method(&mut self, method_id: MethodId) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_method(pin_mut, method_id) };
    }

    /// Get the return code from the message header.
    pub fn get_return_code(&self) -> ReturnCode {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let return_code = unsafe { vsomeip_sys::message_base::get_return_code(&message_base) };
        return_code.into()
    }

    /// Sets the return code in the message header.
    pub fn set_return_code(&mut self, return_code: ReturnCode) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_return_code(pin_mut, return_code.into()) };
    }

    /// Get the interface version from the message header.
    pub fn get_interface_version(&self) -> InterfaceVersion {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_interface_version(&message_base) }
    }

    /// Sets the interface version in the message header.
    pub fn set_interface_version(&mut self, interface_version: InterfaceVersion) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_interface_version(pin_mut, interface_version) };
    }

    /// Get the protocol version from the message header.
    ///
    /// As the protocol version is a fixed value for a vsomeip implementation,
    /// it cannot be set.
    pub fn get_protocol_version(&self) -> ProtocolVersion {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::get_protocol_version(&message_base) }
    }

    /// Get the message type from the message header.
    pub fn get_message_type(&self) -> MessageType {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let message_type = unsafe { vsomeip_sys::message_base::get_message_type(&message_base) };
        message_type.into()
    }

    /// Set the message type in the message header.
    pub fn set_message_type(&mut self, message_type: MessageType) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_message_type(pin_mut, message_type.into()) };
    }

    /// Return the transport mode that was/will be used to send the message.
    pub fn is_reliable(&self) -> bool {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        unsafe { vsomeip_sys::message_base::is_reliable(&message_base) }
    }

    /// Set the transport mode that will be used to send the message.    
    pub fn set_reliable(&mut self, reliable: bool) {
        let message_base = unsafe { vsomeip_sys::as_message_base(&self.inner) };
        let pin_mut = unsafe { util::shared_to_pin(&message_base) };
        unsafe { vsomeip_sys::message_base::set_reliable(pin_mut, reliable) };
    }

    /// Returns a message payload.
    pub fn get_payload(&self) -> Payload {
        let payload = unsafe { self.inner.get_payload() };

        Payload {
            inner: payload
        }
    }

    /// Sets the message payload.
    pub fn set_payload(&mut self, payload: &Payload) {
        unsafe { vsomeip_sys::message::set_payload(self.pin_mut(), payload.inner.clone()) };
    }
}

// Write tests to check the functionality of the message module
// Write meaningful tests to check the functionality of the message module
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ReturnCode, MessageType};

    #[test]
    fn test_message() {
        let mut message = Message::new(true);

        assert_eq!(message.get_service(), 0);
        assert_eq!(message.get_instance(), 0);
        assert_eq!(message.get_client(), 0);
        assert_eq!(message.get_session(), 0);
        assert_eq!(message.get_interface_version(), 0);
        assert_eq!(message.get_protocol_version(), 1);
        assert_eq!(message.get_method(), 0);
        assert_eq!(message.get_return_code(), ReturnCode::Ok);
        assert_eq!(message.get_message_type(), MessageType::Unknown);

        let payload = Payload::with_data("Hello".as_bytes());
        message.set_payload(&payload);
        assert_eq!(message.get_payload().get_data(), "Hello".as_bytes());

        message.set_service(1234);
        assert_eq!(message.get_service(), 1234);

        message.set_client(5678);
        assert_eq!(message.get_client(), 5678);

        message.set_session(9012);
        assert_eq!(message.get_session(), 9012);

        message.set_interface_version(123);
        assert_eq!(message.get_interface_version(), 123);

        message.set_instance(4567);
        assert_eq!(message.get_instance(), 4567);

        message.set_method(890);
        assert_eq!(message.get_method(), 890);

        message.set_return_code(ReturnCode::Ok);
        assert_eq!(message.get_return_code(), ReturnCode::Ok);
    }

    #[test]
    pub fn test_request_response() {
        let request = Message::request(true);
        let response = Message::response(&request);

        assert_eq!(request.get_service(), 0);
        assert_eq!(request.get_instance(), 0);
        assert_eq!(request.get_method(), 0);
        assert_eq!(request.get_return_code(), ReturnCode::Ok);
        assert_eq!(request.get_message_type(), MessageType::Request);

        assert_eq!(response.get_service(), 0);
        assert_eq!(response.get_instance(), 0);
        assert_eq!(response.get_method(), 0);
        assert_eq!(response.get_return_code(), ReturnCode::Ok);
        assert_eq!(response.get_message_type(), MessageType::Response);
    }
}
