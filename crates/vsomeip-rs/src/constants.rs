use crate::*;
// Based on 
// https://github.com/COVESA/vsomeip/blob/master/interface/vsomeip/constants.hpp
// Ideally we want this to be generated automatically by autocxx

pub const DEFAULT_MAJOR: MajorVersion = 0x00;
pub const DEFAULT_MINOR: MinorVersion = 0x00000000;
pub const DEFAULT_TTL: u32 = 0xFFFFFF; // "until next reboot"

pub const DEFAULT_MULTICAST: &str = "224.0.0.0";
pub const DEFAULT_PORT: u16 = 30500;
pub const ILLEGAL_PORT: u16 = 0xFFFF;
pub const ANY_PORT: u16 = 0;

pub const NO_TRACE_FILTER_EXPRESSION: u16 = 0x0000;

pub const ANY_SERVICE: ServiceId = 0xFFFF;
pub const ANY_INSTANCE: InstanceId = 0xFFFF;
pub const ANY_EVENTGROUP: EventGroupId = 0xFFFF;
pub const ANY_METHOD: MethodId = 0xFFFF;
pub const ANY_MAJOR: MajorVersion = 0xFF;
pub const ANY_MINOR: MinorVersion = 0xFFFFFFFF;

pub const DEFAULT_EVENTGROUP: EventGroupId = 0x0001;

pub const ILLEGAL_CLIENT: ClientId = 0x0000;
pub const INVALID_METHOD: MethodId = 0x0000;

pub const MAGIC_COOKIE_CLIENT_MESSAGE: u8 = 0x00;
pub const MAGIC_COOKIE_SERVICE_MESSAGE: u8 = 0x80;
pub const MAGIC_COOKIE_SIZE: u32 = 0x00000008;
pub const MAGIC_COOKIE_REQUEST: u32 = 0xDEADBEEF;
pub const MAGIC_COOKIE_CLIENT: u16 = 0xDEAD;
pub const MAGIC_COOKIE_PROTOCOL_VERSION: u8 = 0x01;
pub const MAGIC_COOKIE_INTERFACE_VERSION: u8 = 0x01;
pub const MAGIC_COOKIE_CLIENT_MESSAGE_TYPE: MessageType = MessageType::RequestNoReturn;
pub const MAGIC_COOKIE_SERVICE_MESSAGE_TYPE: MessageType = MessageType::Notification;
pub const MAGIC_COOKIE_RETURN_CODE: ReturnCode = ReturnCode::Ok;

pub const CLIENT_COOKIE: [u8; 16] = [0xFF, 0xFF, 0x00, 0x00, 0x00, 0x00, 0x00, 0x08,
        0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x01, 0x01, 0x00];

pub const SERVICE_COOKIE: [u8; 16] = [0xFF, 0xFF, 0x80, 0x00, 0x00, 0x00, 0x00,
        0x08, 0xDE, 0xAD, 0xBE, 0xEF, 0x01, 0x01, 0x02, 0x00];

pub const ANY_EVENT: EventId = 0xFFFF;
pub const ANY_CLIENT: ClientId = 0xFFFF;

pub const VSOMEIP_ALL: i32 = -1;

pub const DEFAULT_SECURITY_UPDATE_ID: u32 = 0x0;