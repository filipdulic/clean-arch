use thiserror::Error;

#[derive(Debug, Error)]
pub enum GenError {
    #[error("Token repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum VerifyError {
    #[error("Token not found")]
    NotFound,
    #[error("Token repository connection problem")]
    Connection,
    #[error("Token mismatch")]
    Mismatch,
}

#[derive(Debug)]
pub struct Record {
    pub token: String,
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn gen(&self, email: &str) -> Result<Record, GenError>;
    fn verify(&self, email: &str, token: &str) -> Result<(), VerifyError>;
}
