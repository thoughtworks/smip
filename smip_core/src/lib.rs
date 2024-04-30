pub trait Service {}

pub trait Message {}

pub trait Payload {}

pub struct Configuration {
    
}
pub trait Backend {
    type M: Message
    type P: Payload
}
