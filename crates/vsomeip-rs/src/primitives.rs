pub type ServiceId = vsomeip_sys::service_t;
pub type MethodId = vsomeip_sys::method_t;
pub type InstanceId = vsomeip_sys::instance_t;
pub type MajorVersion = vsomeip_sys::major_version_t;
pub type MinorVersion = vsomeip_sys::minor_version_t;
pub type EventId = vsomeip_sys::event_t;

pub type RequestId = vsomeip_sys::request_t;
pub type ClientId = vsomeip_sys::client_t;
pub type SessionId = vsomeip_sys::session_t;

pub type ProtocolVersion = vsomeip_sys::protocol_version_t;
pub type InterfaceVersion = vsomeip_sys::interface_version_t;

pub type DiagnosisId = vsomeip_sys::diagnosis_t;
pub type EventGroupId = vsomeip_sys::eventgroup_t;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReturnCode {
    Ok,
    NotOk,
    UnknownService,
    UnknownMethod,
    NotReady,
    NotReachable,
    Timeout,
    WrongProtocolVersion,
    WrongInterfaceVersion,
    MalformedMessage,
    WrongMessageType,
    Unknown,
}

impl From<vsomeip_sys::return_code_e> for ReturnCode {
    fn from(value: vsomeip_sys::return_code_e) -> Self {
        match value {
            vsomeip_sys::return_code_e::E_OK => Self::Ok,
            vsomeip_sys::return_code_e::E_NOT_OK => Self::NotOk,
            vsomeip_sys::return_code_e::E_UNKNOWN_SERVICE => Self::UnknownService,
            vsomeip_sys::return_code_e::E_UNKNOWN_METHOD => Self::UnknownMethod,
            vsomeip_sys::return_code_e::E_NOT_READY => Self::NotReady,
            vsomeip_sys::return_code_e::E_NOT_REACHABLE => Self::NotReachable,
            vsomeip_sys::return_code_e::E_TIMEOUT => Self::Timeout,
            vsomeip_sys::return_code_e::E_WRONG_PROTOCOL_VERSION => Self::WrongProtocolVersion,
            vsomeip_sys::return_code_e::E_WRONG_INTERFACE_VERSION => Self::WrongInterfaceVersion,
            vsomeip_sys::return_code_e::E_MALFORMED_MESSAGE => Self::MalformedMessage,
            vsomeip_sys::return_code_e::E_WRONG_MESSAGE_TYPE => Self::WrongMessageType,
            vsomeip_sys::return_code_e::E_UNKNOWN => Self::Unknown,
        }
    }
}

impl Into<vsomeip_sys::return_code_e> for ReturnCode {
    fn into(self) -> vsomeip_sys::return_code_e {
        match self {
            ReturnCode::Ok => vsomeip_sys::return_code_e::E_OK,
            ReturnCode::NotOk => vsomeip_sys::return_code_e::E_NOT_OK,
            ReturnCode::UnknownService => vsomeip_sys::return_code_e::E_UNKNOWN_SERVICE,
            ReturnCode::UnknownMethod => vsomeip_sys::return_code_e::E_UNKNOWN_METHOD,
            ReturnCode::NotReady => vsomeip_sys::return_code_e::E_NOT_READY,
            ReturnCode::NotReachable => vsomeip_sys::return_code_e::E_NOT_REACHABLE,
            ReturnCode::Timeout => vsomeip_sys::return_code_e::E_TIMEOUT,
            ReturnCode::WrongProtocolVersion => vsomeip_sys::return_code_e::E_WRONG_PROTOCOL_VERSION,
            ReturnCode::WrongInterfaceVersion => vsomeip_sys::return_code_e::E_WRONG_INTERFACE_VERSION,
            ReturnCode::MalformedMessage => vsomeip_sys::return_code_e::E_MALFORMED_MESSAGE,
            ReturnCode::WrongMessageType => vsomeip_sys::return_code_e::E_WRONG_MESSAGE_TYPE,
            ReturnCode::Unknown => vsomeip_sys::return_code_e::E_UNKNOWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageType {
    Request,
    RequestNoReturn,
    Notification,
    RequestAck,
    RequestNoReturnAck,
    NotificationAck,
    Response,
    Error,
    ResponseAck,
    ErrorAck,
    Unknown,
}

impl From<vsomeip_sys::message_type_e> for MessageType {
    fn from(value: vsomeip_sys::message_type_e) -> Self {
        match value {
            vsomeip_sys::message_type_e::MT_REQUEST => Self::Request,
            vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN => Self::RequestNoReturn,
            vsomeip_sys::message_type_e::MT_NOTIFICATION => Self::Notification,
            vsomeip_sys::message_type_e::MT_REQUEST_ACK => Self::RequestAck,
            vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN_ACK => Self::RequestNoReturnAck,
            vsomeip_sys::message_type_e::MT_NOTIFICATION_ACK => Self::NotificationAck,
            vsomeip_sys::message_type_e::MT_RESPONSE => Self::Response,
            vsomeip_sys::message_type_e::MT_ERROR => Self::Error,
            vsomeip_sys::message_type_e::MT_RESPONSE_ACK => Self::ResponseAck,
            vsomeip_sys::message_type_e::MT_ERROR_ACK => Self::ErrorAck,
            vsomeip_sys::message_type_e::MT_UNKNOWN => Self::Unknown,
        }
    }
}
impl Into<vsomeip_sys::message_type_e> for MessageType {
    fn into(self) -> vsomeip_sys::message_type_e {
        match self {
            MessageType::Request => vsomeip_sys::message_type_e::MT_REQUEST,
            MessageType::RequestNoReturn => vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN,
            MessageType::Notification => vsomeip_sys::message_type_e::MT_NOTIFICATION,
            MessageType::RequestAck => vsomeip_sys::message_type_e::MT_REQUEST_ACK,
            MessageType::RequestNoReturnAck => vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN_ACK,
            MessageType::NotificationAck => vsomeip_sys::message_type_e::MT_NOTIFICATION_ACK,
            MessageType::Response => vsomeip_sys::message_type_e::MT_RESPONSE,
            MessageType::Error => vsomeip_sys::message_type_e::MT_ERROR,
            MessageType::ResponseAck => vsomeip_sys::message_type_e::MT_RESPONSE_ACK,
            MessageType::ErrorAck => vsomeip_sys::message_type_e::MT_ERROR_ACK,
            MessageType::Unknown => vsomeip_sys::message_type_e::MT_UNKNOWN,
        }
    }
}