use thiserror::Error;

#[derive(Debug, Error)]
pub enum cRyptoError {
    #[error("Failed to create rand SaltGen : {0}")]
    SaltGenError(String),

    #[error("Not Foundend Encryption Method : {0}")]
    EncrMethod(String),

    #[error("Not Foundend Encoding Type : {0}")]
    EncoType(String),

    #[error("Unknown error occurred")]
    Unknown,
}