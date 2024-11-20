use thiserror::Error;

#[derive(Error, Debug, CandidType, Deserialize)]
pub enum TerraPulseError {
    #[error("Not authorized")]
    NotAuthorized,
    #[error("User not found")]
    UserNotFound,
    #[error("NFT not found")]
    NFTNotFound,
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("System error: {0}")]
    SystemError(String),
}

pub type Result<T> = std::result::Result<T, TerraPulseError>;