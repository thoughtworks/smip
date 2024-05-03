use darling::FromMeta;
use someip_types::{MajorVersion, MinorVersion};

#[derive(Debug, FromMeta)]
pub struct ServiceArgs {
    pub id: u16,
    pub major_version: MajorVersion,
    pub minor_version: MinorVersion
}