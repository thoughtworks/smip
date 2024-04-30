use std::net::IpAddr;

pub struct Header {
}
pub type ServiceId = u16;

pub struct ServiceConfig {
    pub id: ServiceId, 
    pub address: IpAddr,
    pub port: u16,
}

pub trait Service {
    type B: Backend;
}

pub trait Message {
    type B: Backend;
    fn header(&self) -> &Header;
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
    fn create_service(config: &ServiceConfig) -> Self::S;
    fn create_payload(data: &[u8]) -> Self::P;
    fn create_message(header: &Header, payload: Self::P) -> Self::M;
}
