use crate::domain::entity::user::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error("User not found")]
    NotFound,
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum GetAllError {
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("User not found")]
    NotFound,
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug)]
pub struct Record {
    user: User,
}

impl From<User> for Record {
    fn from(user: User) -> Self {
        Self { user }
    }
}

impl From<Record> for User {
    fn from(record: Record) -> Self {
        record.user
    }
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn save(&self, record: impl Into<Record>) -> Result<(), SaveError>;
    fn get(&self, id: impl Into<Id>) -> Result<Record, GetError>;
    fn get_all(&self) -> Result<Vec<User>, GetAllError>;
    fn delete(&self, id: impl Into<Id>) -> Result<(), DeleteError>;
}
