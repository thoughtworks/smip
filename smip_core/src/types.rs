use someip_types::*;

pub type Message = vsomeip_rs::Message;
pub type Application = vsomeip_rs::Application;
pub(crate) struct Method<S> {
    pub id: MethodId,
    pub f: fn(&mut S, &Application, &Message)
}
pub struct MethodsBuilder<S> {
    pub(crate) methods: Vec<Method<S>>
}

impl<S: ServiceDefinition> MethodsBuilder<S> {
    pub fn add_method(&mut self, id: MethodId, f: fn(&mut S, &Application, &Message)) {
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