use crate::*;
use std::{net::{IpAddr, ToSocketAddrs}, sync::Arc};

use parking_lot::Mutex;
use someip_types::InstanceId;
use vsomeip_rs::{State, VSomeIpError};
use vsomeip_compat::*;

#[derive(Debug, Clone)]
pub struct RuntimeConfig {
    name: String,
    id: u16,
    addr: Option<IpAddr>,
    netmask: Option<IpAddr>,
    instance_id: InstanceId,
}

impl RuntimeConfig {
    pub fn new(name: impl AsRef<str>, id: u16, instance_id: InstanceId) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            id,
            addr: None,
            netmask: None,
            instance_id,
        }
    }
    pub fn addr(mut self, addr: impl ToSocketAddrs) -> Self {
        self.addr = Some(addr.to_socket_addrs().expect("Invalid address").next().unwrap().ip());
        self
    }
    pub fn netmask(mut self, netmask: impl ToSocketAddrs) -> Self {
        self.netmask = Some(netmask.to_socket_addrs().expect("Invalid address").next().unwrap().ip());
        self
    }
}

pub struct Runtime {
    config: RuntimeConfig,
    vsomeip_config: VsomeIpConfig,
    service_creators: Vec<Box<dyn FnOnce(&vsomeip_rs::Application, InstanceId)>>,
}

impl Runtime {
    pub fn new(config: RuntimeConfig) -> Self {
        let mut vsomeip_config = VsomeIpConfig::new();
        
        vsomeip_config.app_id = (config.name.clone(), config.id);
        vsomeip_config.instance_id = config.instance_id;

        Self {
            config,
            service_creators: vec![],
            vsomeip_config,
        }
    }
    pub fn service<S: ServiceDefinition + ServiceMethods>(mut self, service: S, port: u16) -> Self {
        let mut builder = MethodsBuilder { methods: vec![] };

        let service_id = S::id();
        let major_version = S::major_version();
        let minor_version = S::minor_version();

        S::register_methods(&mut builder);

        let methods = builder.methods;

        self.vsomeip_config.services.push(VSomeIpServiceConfig {
            id: service_id, 
            conn_type: ConnectionType::Tcp(port),
        });

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
    fn set_addr(&mut self) -> Result<(), VSomeIpError> {

        if self.config.addr.is_some() && self.config.netmask.is_some() {
            return Ok(());
        }

        let default_iface = netdev::get_default_interface().expect("No default network interface found, please provide IP and netmask");
        
        if self.config.addr.is_none() {
            self.vsomeip_config.addr = default_iface.ipv4[0].addr.into();
        }

        if self.config.netmask.is_none() {
            self.vsomeip_config.netmask = default_iface.ipv4[0].netmask().into();
        }

        Ok(())
    }
    pub fn run(mut self) -> Result<(), VSomeIpError> {
        self.set_addr()?;

        let config_str = self.vsomeip_config.build();
        
        println!("{config_str}");
        let app = vsomeip_rs::Runtime::get().create_application_with(self.config.name, |_app| {
            set_vsomeip_config(&config_str);
        })?;

        for creator in self.service_creators {
            (creator)(&app, self.config.instance_id);
        }

        app.start();
        Ok(())
    }
}
