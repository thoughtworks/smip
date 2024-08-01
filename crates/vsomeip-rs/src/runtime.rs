use cxx::*;

use crate::{util::AsPinMut, Application, Message, Payload, VSomeIpError};

/// Singleton class containing all public resource management
/// facilities of vsomeip.
///
/// The methods of this class shall be used to create instances of all the
/// classes needed to facilitate SOME/IP communication. In particular, it is
/// the entry point to create instances of the [`Application`] class that
/// contains the main public API of vsomeip.
///
/// [`Application`]: struct.Application.html
pub struct Runtime {
    inner: SharedPtr<vsomeip_sys::runtime>
}

unsafe impl Send for Runtime {}
unsafe impl Sync for Runtime {}

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
    /// Creates a vsomeip application object.
    ///
    /// An application object manages service offers and requests as well as
    /// event subscriptions. It allows registering user application functions
    /// as callbacks that are called on specific events during runtime, e.g.,
    /// to react to incoming SOME/IP messages.
    /// An application object is identified by a unique name that is also used
    /// in (and therefore has to match) the configuration files of vsomeip. If
    /// the name is left empty, the application name is taken from the
    /// environment variable "VSOMEIP_APPLICATION_NAME".
    /// 
    /// Note: The application object is also initialized after creation by calling `app.init()`.
    /// This is where the Rust api differs from C++, to prevent the user from forgetting initialize the application before using it.
    ///
    /// # Parameters
    /// - `name`: Name of the application on the system.
    pub fn create_application_with_name(&self, name: impl AsRef<str>) -> Result<Application, VSomeIpError> {
        self.create_application_with(name, |_| {})
    }
    /// Creates a vsomeip application object.
    /// 
    /// This function allows to pass a closure that will be called before the application is initialized.

    /// # Parameters
    /// - `name`: Name of the application on the system.
    /// - `pre_init`: Closure that will be called before the application is initialized.
    pub fn create_application_with(&self, name: impl AsRef<str>, pre_init: impl FnOnce(&Application)) -> Result<Application, VSomeIpError> {
        let name = name.as_ref();
        let_cxx_string!(name_cxx = name);
        let application = unsafe { vsomeip_sys::runtime::create_application(self.pin_mut(), &name_cxx) }; 

        let app = Application {
            inner: application,
        };

        (pre_init)(&app);

        if !app.init() {
            return Err(VSomeIpError::ApplicationInitError);
        }

        Ok(app)
    }

    /// Creates an empty payload object.
    pub fn create_payload(&self) -> Payload {
        let inner = unsafe { self.inner.create_payload() };
        
        Payload {
            inner
        }
    }
    /// Constructs an empty message object.
    ///
    /// The message can then be used to call [`Application::send`] to send a
    /// SOME/IP message. The user application is responsible for setting
    /// the message type, the service instance, and the message payload
    /// after this call and before calling [`Application::send`].
    ///
    /// # Parameters
    /// - `reliable`: Determines whether this message shall be sent
    ///   over a reliable connection (TCP) or not (UDP).
    ///
    /// [`Application::send`]: struct.Application.html#method.send
    pub fn create_message(&self, reliable: bool) -> Message {
        let inner = unsafe { self.inner.create_message(reliable) };

        Message {
            inner
        }
    }
    /// Constructs an empty request message.
    ///
    /// The message can then be used to call [`Application::send`] to send a
    /// SOME/IP message. The message type is set to REQUEST after the
    /// call and the request identifier is automatically set during the
    /// [`Application::send`] call.
    ///
    /// The user application is responsible for setting the service instance
    /// and the payload.
    ///
    /// # Parameters
    /// - `reliable`: Determines whether this message shall be sent
    ///   over a reliable connection (TCP) or not (UDP).
    ///
    /// [`Application::send`]: struct.Application.html#method.send
    pub fn create_request(&self, reliable: bool) -> Message {
        let raw_message = unsafe { vsomeip_sys::runtime::create_request(&self.inner, reliable) };

        Message {
            inner: raw_message
        }
    }
    /// Constructs an empty response message from a given request message.
    ///
    /// The message can then be used to call [`Application::send`] to send a
    /// SOME/IP message. The message type is set to RESPONSE after the
    /// call and the request identifier is automatically set from the
    /// request message.
    ///
    /// The user application is responsible for setting the service instance
    /// and the payload.
    ///
    /// # Parameters
    /// - `request`: The request message that shall be answered by
    ///   the response message.
    ///
    /// [`Application::send`]: struct.Application.html#method.send
    pub fn create_response(&self, request: &Message) -> Message {
        let raw_message = unsafe {vsomeip_sys::runtime::create_response(&self.inner, request.inner())};

        Message {
            inner: raw_message
        }
    }
}