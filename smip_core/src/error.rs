use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmipError {
    #[error("Failed to parse payload {0}")]
    FromPayloadError(bincode::Error),
    #[error("Failed to write payload {0}")]
    ToPayloadError(bincode::Error),
    #[error("No response received")]
    NoResponse
}