pub type MessageId = u32;
pub type ServiceId = u16;
pub type InstanceId = u16;
pub type MethodId = u16;

pub type Length = u32;

pub type RequestId = u32;
pub type ClientId = u16;
pub type SessionId = u16;

pub type ProtocolVersion = u8;
pub type InterfaceVersion = u8;
pub type MajorVersion = u8;
pub type MinorVersion = u32;
pub type Port = u16;

#[derive(PartialEq, Eq, Debug, Hash, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct SOMEIpHeader {
    pub service_id: ServiceId,
    pub method_id: MethodId,
    pub client_id: ClientId,
    pub session_id: SessionId,
    pub protocol_version: ProtocolVersion,
    pub interface_version: InterfaceVersion,
    pub message_type: MessageType,
    pub return_code: ReturnCode,
}

/// Used to signal whether a request was successfully processed.
#[repr(u8)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum ReturnCode {
    #[default]
    Ok = 0x00,
    NotOk = 0x01,
    UnknownService = 0x02,
    UnknownMethod = 0x03,
    NotReady = 0x04,
    NotReachable = 0x05,
    Timeout = 0x06,
    WrongProtocolVersion = 0x07,
    WrongInterfaceVersion = 0x08,
    MalformedMessage = 0x09,
    WrongMessageType = 0x0a,
    E2eRepeated = 0x0b,
    E2eWrongSequence = 0x0c,
    E2e = 0x0d,
    E2eNotAvailable = 0x0e,
    E2eNoNewData = 0x0f,
    Reserved(u8),
    Unknown(u8),
}

impl From<u8> for ReturnCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Ok,
            0x01 => Self::NotOk,
            0x02 => Self::UnknownService,
            0x03 => Self::UnknownMethod,
            0x04 => Self::NotReady,
            0x05 => Self::NotReachable,
            0x06 => Self::Timeout,
            0x07 => Self::WrongProtocolVersion,
            0x08 => Self::WrongInterfaceVersion,
            0x09 => Self::MalformedMessage,
            0x0a => Self::WrongMessageType,
            0x0b => Self::E2eRepeated,
            0x0c => Self::E2eWrongSequence,
            0x0d => Self::E2e,
            0x0e => Self::E2eNotAvailable,
            0x0f => Self::E2eNoNewData,
            n if (0x10..=0x5e).contains(&n) => Self::Reserved(n),
            n => Self::Unknown(n),
        }
    }
}

impl From<ReturnCode> for u8 {
    fn from(value: ReturnCode) -> Self {
        match value {
            ReturnCode::Ok => 0x00,
            ReturnCode::NotOk => 0x01,
            ReturnCode::UnknownService => 0x02,
            ReturnCode::UnknownMethod => 0x03,
            ReturnCode::NotReady => 0x04,
            ReturnCode::NotReachable => 0x05,
            ReturnCode::Timeout => 0x06,
            ReturnCode::WrongProtocolVersion => 0x07,
            ReturnCode::WrongInterfaceVersion => 0x08,
            ReturnCode::MalformedMessage => 0x09,
            ReturnCode::WrongMessageType => 0x0a,
            ReturnCode::E2eRepeated => 0x0b,
            ReturnCode::E2eWrongSequence => 0x0c,
            ReturnCode::E2e => 0x0d,
            ReturnCode::E2eNotAvailable => 0x0e,
            ReturnCode::E2eNoNewData => 0x0f,
            ReturnCode::Reserved(x) | ReturnCode::Unknown(x) => x,
        }
    }
}

#[repr(u8)]
#[derive(PartialEq, Eq, Hash, Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum MessageType {
    #[default]
    Request = 0x00,
    RequestNoReturn = 0x01,
    Notification = 0x02,
    Response = 0x80,
    Error = 0x81,
    TpRequest = 0x20,
    TpRequestNoReturn = 0x21,
    TpNotification = 0x22,
    TpResponse = 0xa0,
    TpError = 0xa1,
    Unknown(u8),
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0x00 => Self::Request,
            0x01 => Self::RequestNoReturn,
            0x02 => Self::Notification,
            0x80 => Self::Response,
            0x81 => Self::Error,
            0x20 => Self::TpRequest,
            0x21 => Self::TpRequestNoReturn,
            0x22 => Self::TpNotification,
            0xa0 => Self::TpResponse,
            0xa1 => Self::TpError,
            x => Self::Unknown(x),
        }
    }
}

impl From<MessageType> for u8 {
    fn from(value: MessageType) -> Self {
        match value {
            MessageType::Request => 0x00,
            MessageType::RequestNoReturn => 0x01,
            MessageType::Notification => 0x02,
            MessageType::Response => 0x80,
            MessageType::Error => 0x81,
            MessageType::TpRequest => 0x20,
            MessageType::TpRequestNoReturn => 0x21,
            MessageType::TpNotification => 0x22,
            MessageType::TpResponse => 0xa0,
            MessageType::TpError => 0xa1,
            MessageType::Unknown(x) => x,
        }
    }
}