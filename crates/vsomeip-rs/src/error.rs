use thiserror::Error;

/// Error type for vsomeip-rs
#[derive(Error, Debug)]
pub enum VSomeIpError {
    /// Error during initialization of the application
    #[error("Failed to initialize the application")]
    ApplicationInitError,
}