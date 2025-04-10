use std::future::Future;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum GenError {
    #[error("Token repository connection problem")]
    Connection,
}

#[derive(Debug, Error, Serialize)]
pub enum VerifyError {
    #[error("Token not found")]
    NotFound,
    #[error("Token repository connection problem")]
    Connection,
    #[error("Token mismatch")]
    Mismatch,
    #[error("Token expired")]
    TokenExpired,
}

#[derive(Debug, Error, Serialize)]
pub enum ExtendError {
    #[error("Token repository connection problem")]
    Connection,
    #[error("Token not found")]
    NotFound,
}

#[derive(Debug)]
pub struct Record {
    pub token: String,
}

pub trait Repo: Send + Sync {
    fn gen(&self, email: &str) -> impl Future<Output = Result<Record, GenError>>;
    fn verify(&self, email: &str, token: &str) -> impl Future<Output = Result<(), VerifyError>>;
    fn extend(&self, email: &str) -> impl Future<Output = Result<(), ExtendError>>;
}
