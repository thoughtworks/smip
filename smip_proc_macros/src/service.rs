use darling::FromMeta;

#[derive(Debug, FromMeta)]
pub struct ServiceArgs {
    pub id: u16,
    pub major_version: u8,
    pub minor_version: u32
}