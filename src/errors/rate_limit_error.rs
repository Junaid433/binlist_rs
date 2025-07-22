use thiserror::Error;

#[derive(Error, Debug)]
#[error("{status_code}: Rate limit exceeded. Please try again later.")]
pub struct RateLimitExceeded {
    pub status_code: u16,
}

impl RateLimitExceeded {
    pub fn new(status_code: u16) -> Self {
        RateLimitExceeded { status_code }
    }
}
