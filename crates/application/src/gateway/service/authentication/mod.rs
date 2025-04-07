use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum AuthenticationServiceError {
    #[error("Authentication token expired")]
    TokenExpired,
    #[error("Failed to retrive claims from token")]
    ClaimsRetrievalFailed,
    #[error("Invalid token")]
    InvalidToken,
}

pub trait AuthenticationService {
    fn authenticate(&self, token: String) -> Result<Claims, AuthenticationServiceError>;
}
