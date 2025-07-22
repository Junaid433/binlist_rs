use thiserror::Error; 
use crate::errors::rate_limit_error::RateLimitExceeded;

#[derive(Error, Debug)]
pub enum LookupError {
    #[error("Failed to look up BIN {0}: HTTP {1}")]
    BINLookupError(String, u16),

    #[error("HTTP error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    #[error(transparent)]
    RateLimitExceeded(#[from] RateLimitExceeded),
}