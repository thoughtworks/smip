use std::collections::HashMap;

use crate::*;
pub struct VSomeIPService {
    config: ServiceConfig,
    methods: HashMap<MethodId, Box<dyn FnMut(vsomeip_rs::Message) + 'static>>
}

impl Service for VSomeIPService {
    type B = vsomeip_rs::Application;
    
    fn with_method<F>(&mut self, method_id: MethodId, handler: F) where F: FnMut(<Self::B as Backend>::M) + 'static {
        self.methods.insert(method_id, Box::new(handler));
    }
}

impl Backend for vsomeip_rs::Application {
    type S = VSomeIPService;
    type M = vsomeip_rs::Message;
    type P = vsomeip_rs::Payload;
    
    fn init(name: impl AsRef<str>) -> Self {
        vsomeip_rs::Runtime::get().create_application_with_name(name)
    }
    fn create_service(config: &ServiceConfig) -> Self::S {
        VSomeIPService {
            config: config.clone(),
            methods: HashMap::new()
        }
    }
    fn offer_service(&self, service: &Self::S) {
        self.offer_service(service.config.id, service.config.instance_id, service.config.major_version, service.config.minor_version);
        
        for (method_id, handler) in service.methods {
            self.register_message_handler(service_id, instance_id, method_id, f)
        }
    }
    fn create_payload(data: &[u8]) -> Self::P {
        vsomeip_rs::Payload::with_data(data)
    }
    fn create_message(header: &SOMEIpHeader, reliable: bool, payload: Self::P) -> Self::M {
        let mut message = vsomeip_rs::Message::new(reliable);

        message.set_payload(&payload);
        message.set_service(header.service_id);
        message.set_method(header.method_id);
        message.set_return_code(header.return_code.into());
        // message.set_message_type();
        // message.set_client();
        // message
    }
}


impl Payload for vsomeip_rs::Payload {
    type B = vsomeip_rs::Application;
    
    fn data(&self) -> &[u8] {
        self.get_data()
    }

    fn with_data(data: &[u8]) -> Self where Self: Sized {
        vsomeip_rs::Payload::with_data(data)
    }
}

impl Message for vsomeip_rs::Message {
    type B = vsomeip_rs::Application;
    
    fn header(&self) -> &S {
        // self.header()
    }
    fn payload(&self) -> &vsomeip_rs::Payload {
        self.get_payload()
    }
}
