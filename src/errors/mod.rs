pub mod lookup_error; 
pub mod rate_limit_error;

pub use lookup_error::LookupError;
pub use rate_limit_error::RateLimitExceeded;