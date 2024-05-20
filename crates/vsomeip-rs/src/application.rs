use cxx::*;

use crate::{primitives::{InstanceId, MajorVersion, MethodId, MinorVersion, ServiceId}, util::AsPinMut, Message, State};

/// This class contains the public API of the vsomeip implementation.
///
/// Due to its heavy resource footprint, it should exist once per client and can
/// be instantiated using the API of [`Runtime`]. It manages the lifecycle of
/// the vsomeip client and allocates all resources needed to communicate.
///
/// [`Runtime`]: struct.Runtime.html
#[derive(Clone)]
pub struct Application {
    pub(crate) inner: SharedPtr<vsomeip_sys::application>,
}

unsafe impl Send for Application {}
unsafe impl Sync for Application {}

unsafe impl AsPinMut for Application {
    type Inner = vsomeip_sys::application;

    fn inner(&self) -> &SharedPtr<Self::Inner> {
        &self.inner
    }
}

impl Application {
    /// Returns the name of the application as given during creation.
    ///
    /// The application name is used to identify the application. It is either
    /// set explicitly when the application object is created or configured by
    /// the environment variable `VSOMEIP_APPLICATION_NAME`.
    ///
    /// Note: A user application can use several vsomeip application objects in
    /// parallel. The application names must be set explicitly in this case
    /// because `VSOMEIP_APPLICATION_NAME` only allows specifying a single name.
    ///
    /// # Returns
    /// Application name
    pub fn name(&self) -> &str {
        let name_cxx = unsafe { self.inner.get_name() };

        name_cxx.to_str().unwrap()
    }
    
    /// Initializes the application.
    ///
    /// The `init` method must be called first after creating a vsomeip
    /// application and executes the following steps to initialize it:
    /// - Loading the configuration from a dynamic module
    ///   - Loading the configuration from a `.json` file or
    ///   - Loading the configuration from compiled data (not yet available)
    /// - Determining routing configuration and initialization of the routing
    ///   itself
    /// - Installing signal handlers
    pub(crate) fn init(&self) -> bool {
        unsafe { vsomeip_sys::application::init(self.pin_mut()) }
    }

    /// Starts message processing.
    ///
    /// This method must be called after `init` to start message processing. It
    /// will block until the message processing is terminated using the `stop`
    /// method or by receiving signals. It processes messages received
    /// via the sockets and uses registered callbacks to pass them to the user
    /// application.
    pub fn start(&self) {
        unsafe { vsomeip_sys::application::start(self.pin_mut()) }
    }

    /// Stops message processing.
    ///
    /// This method stops message processing. Thus, `start` will return
    /// after a call to `stop`.
    pub fn stop(&self) {
        unsafe { vsomeip_sys::application::stop(self.pin_mut()) }
    }

    /// Offers a SOME/IP service instance.
    ///
    /// The user application must call this method for each service it offers
    /// to register it at the vsomeip routing component, which makes the
    /// service visible to interested clients. Depending on the configuration,
    /// the service is available internally only or internally and externally.
    /// To offer a service to the external network, the configuration must
    /// contain a port for the offered service instance. If no such port
    /// configuration is provided, the service is not visible outside the
    /// device.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the offered service interface.
    /// - `instance_id`: Instance identifier of the offered service instance.
    /// - `major_version`: Major service version (Default: 0).
    /// - `minor_version`: Minor service version (Default: 0).
    pub fn offer_service(&self, service_id: ServiceId, instance_id: InstanceId, major_version: MajorVersion, minor_version: MinorVersion) {
        unsafe { vsomeip_sys::application::offer_service(self.pin_mut(), service_id, instance_id, major_version, minor_version) }
    }

    /// Stops offering a SOME/IP service instance.
    ///
    /// The user application must call this method to withdraw a service offer.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the offered service interface.
    /// - `instance_id`: Instance identifier of the offered service instance.
    /// - `major_version`: Major service version (Default: 0).
    /// - `minor_version`: Minor service version (Default: 0).
    pub fn stop_offer_service(&self, service_id: ServiceId, instance_id: InstanceId, major_version: MajorVersion, minor_version: MinorVersion) {
        unsafe { vsomeip_sys::application::stop_offer_service(self.pin_mut(), service_id, instance_id, major_version, minor_version) }
    }

    /// Registers the application as a client of a service instance.
    ///
    /// A user application must call this method for each service instance it
    /// wants to use. The request is stored within the routing component and the
    /// application is registered as a client for the service as soon as the
    /// service instance becomes available.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the requested service interface.
    /// - `instance_id`: Instance identifier of the requested service instance.
    /// - `major_version`: Major service version (Default: 0xFF).
    /// - `minor_version`: Minor service version (Default: 0xFFFFFF).
    pub fn request_service(&self, service_id: ServiceId, instance_id: InstanceId, major_version: MajorVersion, minor_version: MinorVersion) {
        unsafe { vsomeip_sys::application::request_service(self.pin_mut(), service_id, instance_id, major_version, minor_version) }
    }

    /// Unregisters the application as a client of a service instance.
    ///
    /// A user application should call this method if it no longer needs to
    /// use the service instance. The method unregisters the request from
    /// the routing component, which removes the service instance from the
    /// list of requested service instances if the call releases the last
    /// existing request for the service instance. This is important for
    /// external service instances, as the SOME/IP Service Discovery can avoid
    /// sending unnecessary Find messages.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the offered service interface.
    /// - `instance_id`: Instance identifier of the offered service instance.
    pub fn release_service(&self, service_id: ServiceId, instance_id: InstanceId) {
        unsafe { vsomeip_sys::application::release_service(self.pin_mut(), service_id, instance_id) }
    }

    /// Sends a message.
    ///
    /// Serializes the specified message object, determines the target, and sends
    /// the message to the target. For requests, the request identifier is
    /// automatically built from the client identifier and the session
    /// identifier.
    ///
    /// # Parameters
    /// - `message`: Message object.
    pub fn send(&self, message: &Message) {
        unsafe { vsomeip_sys::application::send(self.pin_mut(), message.inner.clone()) }
    }

    /// Register a state handler with the vsomeip runtime.
    ///
    /// The state handler tells if this client is successfully deregistered
    /// at the central vsomeip routing component. This is called during the
    /// `start` and `stop` methods of this class to inform the user
    /// application about the registration state.
    ///
    /// # Parameters
    /// - `handler`: Handler function to be called on state change.
    pub fn register_state_handler<F: FnMut(State) + 'static>(&self, mut handler: F) {
        let state_callback = vsomeip_sys::StateHandlerCallback::from_closure(move |raw_state| {
            let state: State = State::from(raw_state);

            (handler)(state);
        });

        unsafe {
            vsomeip_sys::application_register_state_handler(self.pin_mut(), state_callback.f, state_callback.user_data as *mut _);
        }
    }

    /// Unregister the state handler.
    pub fn unregister_state_handler(&self) {
        unsafe { vsomeip_sys::application::unregister_state_handler(self.pin_mut()); }
    }
    
    /// Registers a handler for the specified method or event.
    ///
    /// A user application must call this method to register callbacks for
    /// messages that match the specified service, instance, method/event
    /// pattern. It is possible to specify wildcard values for all three
    /// identifier arguments.
    ///
    /// Notes:
    /// - Only a single handler can be registered per service, instance,
    ///   method/event combination.
    /// - A subsequent call will overwrite an existing registration.
    /// - Handler registrations containing wildcards can be active in parallel
    ///   to handler registrations for specific service, instance, method/event
    ///   combinations.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the service that contains the
    ///   method or event. Can be set to `ANY_SERVICE` to register a handler for
    ///   a message independent from a specific service.
    /// - `instance_id`: Instance identifier of the service instance that
    ///   contains the method or event. Can be set to `ANY_INSTANCE` to register
    ///   a handler for a message independent from a specific service.
    /// - `method_id`: Method/Event identifier of the method/event that is
    ///   to be handled. Can be set to `ANY_METHOD` to register a handler for
    ///   all methods and events.
    /// - `handler`: Callback that will be called if a message arrives
    ///   that matches the specified service, instance, and method/event
    ///   parameters.
    pub fn register_message_handler<F: FnMut(&Message) + 'static>(&self, service_id: ServiceId, instance_id: InstanceId, method_id: MethodId, mut handler: F) {
        let message_callback = vsomeip_sys::MessageHandlerCallback::from_closure(move |raw_message| {
            let message = Message {
                inner: raw_message
            };
            
            (handler)(&message);
        });
        
        unsafe { 
            vsomeip_sys::application_register_message_handler(self.pin_mut(), service_id, instance_id, method_id, message_callback.f, message_callback.user_data as *mut _); 
        }
    }

    /// Unregisters the message handler for the specified service
    /// method/event notification.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the service that contains the
    ///   method or event. Can be set to `ANY_SERVICE` to unregister a handler for
    ///   a message independent from a specific service.
    /// - `instance_id`: Instance identifier of the service instance that
    ///   contains the method or event. Can be set to `ANY_INSTANCE` to unregister
    ///   a handler for a message independent from a specific service.
    /// - `method_id`: Method/Event identifier of the method/event that is
    ///   to be handled. Can be set to `ANY_METHOD` to unregister a handler for
    ///   all methods and events.
    pub fn unregister_message_handler(&self, service_id: ServiceId, instance_id: InstanceId, method_id: MethodId) {
        unsafe {
            vsomeip_sys::application::unregister_message_handler(self.pin_mut(), service_id, instance_id, method_id);
        }
    }

    /// Register a callback that is called when service instances
    /// availability changes.
    ///
    /// This method allows for the registration of callbacks that are called
    /// whenever a service appears or disappears. It is possible to specify
    /// wildcards for service, instance, and/or version. Additionally, the
    /// version specification is optional and defaults to `DEFAULT_MAJOR`
    /// /`DEFAULT_MINOR`.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the service instance whose
    ///   availability shall be reported. Can be set to `ANY_SERVICE`.
    /// - `instance_id`: Instance identifier of the service instance whose
    ///   availability shall be reported. Can be set to `ANY_INSTANCE`.
    /// - `handler`: Callback to be called if availability changes.
    /// - `major_version`: Major service version. The parameter defaults to
    ///   `DEFAULT_MAJOR` and can be set to `ANY_MAJOR`.
    /// - `minor_version`: Minor service version. The parameter defaults to
    ///   `DEFAULT_MINOR` and can be set to `ANY_MINOR`.
    pub fn register_availability_handler<F: FnMut(ServiceId, InstanceId, bool) + 'static>(&self, service_id: ServiceId, instance_id: InstanceId, mut handler: F, major_version: MajorVersion, minor_version: MinorVersion) {
        let state_callback = vsomeip_sys::AvailabilityHandlerCallback::from_closure(move |service_id, instance_id, is_availabe| {
            (handler)(service_id, instance_id, is_availabe);
        });

        unsafe {
            vsomeip_sys::application_register_availability_handler(self.pin_mut(), service_id, instance_id, state_callback.f, major_version, minor_version, state_callback.user_data as *mut _);
        }
    }
    /// Unregister an availability callback.
    ///
    /// # Parameters
    /// - `service_id`: Service identifier of the service instance whose
    ///   availability shall be reported. Can be set to `ANY_SERVICE`.
    /// - `instance_id`: Instance identifier of the service instance whose
    ///   availability shall be reported. Can be set to `ANY_INSTANCE`.
    /// - `major_version`: Major service version. The parameter defaults to
    ///   `DEFAULT_MAJOR` and can be set to `ANY_MAJOR`.
    /// - `minor_version`: Minor service version. The parameter defaults to
    ///   `DEFAULT_MINOR` and can be set to `ANY_MINOR`.
    pub fn unregister_availability_handler(&self, service_id: ServiceId, instance_id: InstanceId, major_version: MajorVersion, minor_version: MinorVersion) {
        unsafe { vsomeip_sys::application::unregister_availability_handler(self.pin_mut(), service_id, instance_id, major_version, minor_version); }
    }

    /// Unregister all registered handlers.
    pub fn clear_all_handlers(&self) {
        unsafe { vsomeip_sys::application::clear_all_handler(self.pin_mut())}
    }
}
