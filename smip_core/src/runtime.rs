use crate::*;
use std::sync::Arc;

use parking_lot::Mutex;
use someip_types::InstanceId;
use vsomeip_rs::{State, VSomeIpError};

pub struct RuntimeConfig {
    pub name: String,
    pub instance_id: InstanceId,
}

pub struct Runtime {
    config: RuntimeConfig,
    service_creators: Vec<Box<dyn FnOnce(&vsomeip_rs::Application, InstanceId)>>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        Self {
            config,
            service_creators: vec![],
        }
    }
    pub fn service<S: ServiceDefinition + ServiceMethods>(mut self, service: S) -> Self {
        let mut builder = MethodsBuilder { methods: vec![] };

        let service_id = service.id();
        let major_version = service.major_version();
        let minor_version = service.minor_version();

        S::register_methods(&mut builder);

        let methods = builder.methods;

        let creator = move |app: &vsomeip_rs::Application, instance_id: InstanceId| {
            let app_clone = app.clone();

            app.register_state_handler(move |state| {
                if state == State::Registered {
                    app_clone.offer_service(service_id, instance_id, major_version, minor_version);
                }
            });

            let service = Arc::new(Mutex::new(service));

            for method in methods {
                let service_clone = service.clone();
                let app_clone = app.clone();

                app.register_message_handler(service_id, instance_id, method.id, move |message| {
                    let mut service = service_clone.lock();
                    let result = (method.f)(&mut service, &app_clone, &message);
                    dbg!("Here");
                    if let Err(err) = result {
                        //FIXME: Send error response
                        dbg!("{}", err);
                    }
                });
            }
        };

        self.service_creators.push(Box::new(creator));

        self
    }
    pub fn run(self) -> Result<(), VSomeIpError> {
        let app = vsomeip_rs::Runtime::get().create_application_with(self.config.name, |_app| {
            // todo!("Set config")
        })?;

        for creator in self.service_creators {
            (creator)(&app, self.config.instance_id);
        }

        app.start();

        Ok(())
    }
}
