use std::sync::mpsc;

use crate::types::*;
use vsomeip_compat::{set_vsomeip_config, VsomeIpConfig};
use vsomeip_rs::{
    Application, InstanceId, MessageType, MethodId, Runtime, ServiceId, ANY_INSTANCE, ANY_METHOD,
    ANY_SERVICE,
};

pub struct Client {
    application: Application,
    join_handle: Option<std::thread::JoinHandle<()>>,
    receiver: mpsc::Receiver<(ServiceId, InstanceId, MethodId, Payload)>,
}

impl Client {
    pub fn new(config: &VsomeIpConfig) -> anyhow::Result<Self> {
        let runtime = Runtime::get();

        let config_str = config.clone().build();
        let application = runtime.create_application_with(&config.app_id.0, |_app| {
            set_vsomeip_config(&config_str);
        })?;

        let (sender, receiver) = mpsc::channel();

        application.register_message_handler(
            ANY_SERVICE,
            ANY_INSTANCE,
            ANY_METHOD,
            move |message| {
                if message.get_message_type() == MessageType::Response {
                    let service_id = message.get_service();
                    let instance_id = message.get_instance();
                    let method_id = message.get_method();
                    let payload = message.get_payload();

                    sender
                        .send((service_id, instance_id, method_id, payload))
                        .unwrap();
                }
            },
        );

        let application_clone = application.clone();
        let join_handle = std::thread::spawn(move || application_clone.start());
        Ok(Self {
            application,
            join_handle: Some(join_handle),
            receiver,
        })
    }
    pub fn send<T: ToPayload, R: for<'a> FromPayload<'a>>(
        &self,
        service_id: ServiceId,
        instance_id: InstanceId,
        method_id: MethodId,
        data: T,
    ) -> anyhow::Result<R> {
        // self.application
        //     .request_service(service_id, instance_id, 0, 0);

        let mut message = Message::request(true);

        message.set_service(service_id);
        message.set_instance(instance_id);
        message.set_method(method_id);

        let payload = Payload::with_data(&data.to_payload().unwrap());

        message.set_payload(&payload);

        self.application.send(&message);

        println!("Waiting for response");

        for (s_id, i_id, m_id, p) in self.receiver.iter() {
            if s_id == service_id && i_id == instance_id && m_id == method_id {
                println!("Response received");
                return Ok(R::from_payload(p.get_data())?);
            }
        }

        Err(anyhow::anyhow!("No response received"))
    }
}

impl Drop for Client {
    fn drop(&mut self) {
        self.application.stop();
        self.join_handle.take().unwrap().join().unwrap();
    }
}
