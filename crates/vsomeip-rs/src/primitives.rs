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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReturnCode {
    EOk,
    ENotOk,
    EUnknownService,
    EUnknownMethod,
    ENotReady,
    ENotReachable,
    ETimeout,
    EWrongProtocolVersion,
    EWrongInterfaceVersion,
    EMalformedMessage,
    EWrongMessageType,
    EUnknown,
}

impl From<vsomeip_sys::return_code_e> for ReturnCode {
    fn from(value: vsomeip_sys::return_code_e) -> Self {
        match value {
            vsomeip_sys::return_code_e::E_OK => Self::EOk,
            vsomeip_sys::return_code_e::E_NOT_OK => Self::ENotOk,
            vsomeip_sys::return_code_e::E_UNKNOWN_SERVICE => Self::EUnknownService,
            vsomeip_sys::return_code_e::E_UNKNOWN_METHOD => Self::EUnknownMethod,
            vsomeip_sys::return_code_e::E_NOT_READY => Self::ENotReady,
            vsomeip_sys::return_code_e::E_NOT_REACHABLE => Self::ENotReachable,
            vsomeip_sys::return_code_e::E_TIMEOUT => Self::ETimeout,
            vsomeip_sys::return_code_e::E_WRONG_PROTOCOL_VERSION => Self::EWrongProtocolVersion,
            vsomeip_sys::return_code_e::E_WRONG_INTERFACE_VERSION => Self::EWrongInterfaceVersion,
            vsomeip_sys::return_code_e::E_MALFORMED_MESSAGE => Self::EMalformedMessage,
            vsomeip_sys::return_code_e::E_WRONG_MESSAGE_TYPE => Self::EWrongMessageType,
            vsomeip_sys::return_code_e::E_UNKNOWN => Self::EUnknown,
        }
    }
}

impl Into<vsomeip_sys::return_code_e> for ReturnCode {
    fn into(self) -> vsomeip_sys::return_code_e {
        match self {
            ReturnCode::EOk => vsomeip_sys::return_code_e::E_OK,
            ReturnCode::ENotOk => vsomeip_sys::return_code_e::E_NOT_OK,
            ReturnCode::EUnknownService => vsomeip_sys::return_code_e::E_UNKNOWN_SERVICE,
            ReturnCode::EUnknownMethod => vsomeip_sys::return_code_e::E_UNKNOWN_METHOD,
            ReturnCode::ENotReady => vsomeip_sys::return_code_e::E_NOT_READY,
            ReturnCode::ENotReachable => vsomeip_sys::return_code_e::E_NOT_REACHABLE,
            ReturnCode::ETimeout => vsomeip_sys::return_code_e::E_TIMEOUT,
            ReturnCode::EWrongProtocolVersion => vsomeip_sys::return_code_e::E_WRONG_PROTOCOL_VERSION,
            ReturnCode::EWrongInterfaceVersion => vsomeip_sys::return_code_e::E_WRONG_INTERFACE_VERSION,
            ReturnCode::EMalformedMessage => vsomeip_sys::return_code_e::E_MALFORMED_MESSAGE,
            ReturnCode::EWrongMessageType => vsomeip_sys::return_code_e::E_WRONG_MESSAGE_TYPE,
            ReturnCode::EUnknown => vsomeip_sys::return_code_e::E_UNKNOWN,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageType {
    MTRequest,
    MTRequestNoReturn,
    MTNotification,
    MTRequestAck,
    MTRequestNoReturnAck,
    MTNotificationAck,
    MTResponse,
    MTError,
    MTResponseAck,
    MTErrorAck,
    MTUnknown,
}

impl From<vsomeip_sys::message_type_e> for MessageType {
    fn from(value: vsomeip_sys::message_type_e) -> Self {
        match value {
            vsomeip_sys::message_type_e::MT_REQUEST => Self::MTRequest,
            vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN => Self::MTRequestNoReturn,
            vsomeip_sys::message_type_e::MT_NOTIFICATION => Self::MTNotification,
            vsomeip_sys::message_type_e::MT_REQUEST_ACK => Self::MTRequestAck,
            vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN_ACK => Self::MTRequestNoReturnAck,
            vsomeip_sys::message_type_e::MT_NOTIFICATION_ACK => Self::MTNotificationAck,
            vsomeip_sys::message_type_e::MT_RESPONSE => Self::MTResponse,
            vsomeip_sys::message_type_e::MT_ERROR => Self::MTError,
            vsomeip_sys::message_type_e::MT_RESPONSE_ACK => Self::MTResponseAck,
            vsomeip_sys::message_type_e::MT_ERROR_ACK => Self::MTErrorAck,
            vsomeip_sys::message_type_e::MT_UNKNOWN => Self::MTUnknown,
        }
    }
}
impl Into<vsomeip_sys::message_type_e> for MessageType {
    fn into(self) -> vsomeip_sys::message_type_e {
        match self {
            MessageType::MTRequest => vsomeip_sys::message_type_e::MT_REQUEST,
            MessageType::MTRequestNoReturn => vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN,
            MessageType::MTNotification => vsomeip_sys::message_type_e::MT_NOTIFICATION,
            MessageType::MTRequestAck => vsomeip_sys::message_type_e::MT_REQUEST_ACK,
            MessageType::MTRequestNoReturnAck => vsomeip_sys::message_type_e::MT_REQUEST_NO_RETURN_ACK,
            MessageType::MTNotificationAck => vsomeip_sys::message_type_e::MT_NOTIFICATION_ACK,
            MessageType::MTResponse => vsomeip_sys::message_type_e::MT_RESPONSE,
            MessageType::MTError => vsomeip_sys::message_type_e::MT_ERROR,
            MessageType::MTResponseAck => vsomeip_sys::message_type_e::MT_RESPONSE_ACK,
            MessageType::MTErrorAck => vsomeip_sys::message_type_e::MT_ERROR_ACK,
            MessageType::MTUnknown => vsomeip_sys::message_type_e::MT_UNKNOWN,
        }
    }
}