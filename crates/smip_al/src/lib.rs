// mod implementations;

use std::net::IpAddr;
use someip_types::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    #[default]
    Tcp,
    Udp
}

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub name: String,
    pub id: ServiceId, 
    pub instance_id: InstanceId,
    pub address: IpAddr,
    pub major_version: MajorVersion,
    pub minor_version: MinorVersion,
    pub port: u16,
    pub connection_type: ConnectionType
}

pub trait Service {
    type B: Backend;
    fn with_method<F>(&mut self, method_id: MethodId, handler: F) where F: FnMut(<Self::B as Backend>::M) + 'static;
}

pub trait Message {
    type B: Backend;
    fn header(&self) -> &SOMEIpHeader;
    fn payload(&self) -> &<Self::B as Backend>::P;
}

pub trait Payload {
    type B: Backend;
    fn with_data(data: &[u8]) -> Self where Self: Sized;
    fn data(&self) -> &[u8];
}

pub trait Backend {
    type S: Service;
    type M: Message;
    type P: Payload;
    fn init(name: impl AsRef<str>) -> Self;
    fn create_service(config: &ServiceConfig) -> Self::S;
    fn offer_service(&self, service: &Self::S);
    fn create_payload(data: &[u8]) -> Self::P;
    fn create_message(header: &SOMEIpHeader, reliable: bool, payload: Self::P) -> Self::M;
}


// vsomeip-rs
// application.init()


// let service1 = Service::new(config).method(method_id, |message| {...});
// backend.offer_service(service1);
// backend.start();