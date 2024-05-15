use std::{sync::{mpsc, Arc}, thread};

use crate::{error::SmipError, types::*};
use parking_lot::{Condvar, Mutex};
use vsomeip_compat::{set_vsomeip_config, VsomeIpConfig};
use vsomeip_rs::{
    Application, InstanceId, MessageType, MethodId, Runtime, ServiceId, ANY_METHOD
};

pub struct Client {
    application: Application,
    join_handle: Option<std::thread::JoinHandle<()>>,
    message_receiver: mpsc::Receiver<(ServiceId, InstanceId, MethodId, Payload)>,
    message_sender: mpsc::Sender<Message>,
    service_id: ServiceId,
    instance_id: InstanceId
}

impl Client {
    fn sender_thread(pair: Arc<(Mutex<bool>, Condvar)>, message_receiver: mpsc::Receiver<Message>, service: ServiceId, instance: InstanceId, application: Application){
        let &(ref lock, ref cvar) = &*pair;
        let mut started = lock.lock();
        if !*started {
            cvar.wait(&mut started);
        }

        for message in message_receiver.iter() {
            // println!("New message to send {:?} {:?}", message.get_message_type(), message.get_payload().get_data());
            // println!("{} {} | {} {}", service, instance, message.get_service(), message.get_instance());
            if message.get_service() == service && message.get_instance() == instance {
                application.send(&message);
            }
        }
    }
    pub fn new(config: &VsomeIpConfig) -> anyhow::Result<Self> {
        let runtime = Runtime::get();

        let config_str = config.clone().build();
        let application = runtime.create_application_with(&config.app_id.0, |_app| {
            set_vsomeip_config(&config_str);
        })?;

        let (sender, receiver) = mpsc::channel();
        let (message_sender, message_receiver) = mpsc::channel::<Message>();


    assert!(config.services.len() == 1);
    let service_id = config.services[0].id;
    let instance_id = config.instance_id;
    
        application.register_message_handler(
            service_id,
            instance_id,
            ANY_METHOD,
            move |message| {
                // println!("New Message");
                if message.get_message_type() == MessageType::Response {
                    // println!("New Response");
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
    let application_clone1 = application.clone();

    application.request_service(service_id, instance_id, 0, 0);

    let pair = Arc::new((Mutex::new(false), Condvar::new()));

    let pair_clone = pair.clone();
    
    
    let application_clone = application.clone();
    thread::spawn(move || {
        Self::sender_thread(pair, message_receiver, service_id, instance_id, application_clone1);
    });


    application.register_availability_handler(service_id, instance_id, move |_service, _instance, _is_available| {
        let &(ref lock, ref cvar) = &*pair_clone;
        let mut started = lock.lock();
        *started = true;
        cvar.notify_one();
    }, 0, 0);

    let join_handle = std::thread::spawn(move || application_clone.start());
    Ok(Self {
        application,
        service_id,
        instance_id,
        join_handle: Some(join_handle),
        message_receiver: receiver,
        message_sender
    })
}
pub fn send<T: ToPayload, R: for<'a> FromPayload<'a>>(
    &self,
    method_id: MethodId,
    data: T,
) -> anyhow::Result<R> {
        let mut message = Message::request(true);

        message.set_service(self.service_id);
        message.set_instance(self.instance_id);
        message.set_method(method_id);

        let payload = Payload::with_data(&data.to_payload().unwrap());

        message.set_payload(&payload);

        self.application.send(&message);
        self.message_sender.send(message).unwrap();

        for (s_id, i_id, m_id, p) in self.message_receiver.iter() {
            if s_id == self.service_id && i_id == self.instance_id && m_id == method_id {
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
