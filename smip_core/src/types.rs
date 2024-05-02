use serde::{Deserialize, Serialize};
use someip_types::*;

pub type Message = vsomeip_rs::Message;
pub type Application = vsomeip_rs::Application;
pub type Payload = vsomeip_rs::Payload;

pub type RequestCallback<S> = fn(&mut S, &Application, &Message) -> anyhow::Result<()>;
pub(crate) struct Method<S> {
    pub id: MethodId,
    pub f: RequestCallback<S>
}
pub struct MethodsBuilder<S> {
    pub(crate) methods: Vec<Method<S>>
}

impl<S: ServiceDefinition> MethodsBuilder<S> {
    pub fn add_method(&mut self, id: MethodId, f: RequestCallback<S>) {
        self.methods.push(Method {id, f});
    }
}
pub trait ServiceDefinition: Send + Sync + 'static {
    fn id(&self) -> ServiceId;
    fn major_version(&self) -> MajorVersion;
    fn minor_version(&self) -> MinorVersion;
}

pub trait ServiceMethods {
    fn register_methods(builder: &mut MethodsBuilder<Self>) where Self: Sized;
}

pub trait FromPayload<'de>: Deserialize<'de> {
    fn from_payload(payload: &'de [u8]) -> Result<Self, bincode::Error> {
        bincode::deserialize(payload)
    }
}


pub trait ToPayload: Serialize {
    fn to_payload(&self) -> Result<Vec<u8>, bincode::Error> {
        bincode::serialize(self)
    }
}

impl<'de, T: Deserialize<'de>> FromPayload<'de> for T {}
impl<T: Serialize> ToPayload for T {}