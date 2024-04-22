use thiserror::Error;

#[derive(Error, Debug)]
pub enum VSomeIpError {
    #[error("Failed to initialize the application")]
    ApplicationInitError,
}