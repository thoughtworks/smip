pub type ServiceId = vsomeip_sys::service_t;
pub type MethodId = vsomeip_sys::method_t;
pub type InstanceId = vsomeip_sys::instance_t;
pub type MajorVersion = vsomeip_sys::major_version_t;
pub type MinorVersion = vsomeip_sys::minor_version_t;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum State {
    Registered,
    Deregistered
}

impl From<vsomeip_sys::state_type_e> for State {
    fn from(value: vsomeip_sys::state_type_e) -> Self {
        match value {
            vsomeip_sys::state_type_e::ST_REGISTERED => Self::Registered,
            vsomeip_sys::state_type_e::ST_DEREGISTERED => Self::Deregistered,
        }
    }
}
impl Into<vsomeip_sys::state_type_e> for State {
    fn into(self) -> vsomeip_sys::state_type_e {
        match self {
            State::Registered => vsomeip_sys::state_type_e::ST_REGISTERED,
            State::Deregistered => vsomeip_sys::state_type_e::ST_DEREGISTERED,
        }
    }
}