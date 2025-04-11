use std::future::Future;

use ca_domain::entity::user::*;
use serde::Serialize;
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

#[derive(Debug, Serialize)]
pub struct Record {
    pub user: User,
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
    fn save(&self, record: Record) -> impl Future<Output = Result<(), SaveError>>;
    fn get(&self, id: Id) -> impl Future<Output = Result<Record, GetError>>;
    fn get_by_username(&self, username: UserName)
        -> impl Future<Output = Result<Record, GetError>>;
    fn get_all(&self) -> impl Future<Output = Result<Vec<Record>, GetAllError>>;
    fn delete(&self, id: Id) -> impl Future<Output = Result<(), DeleteError>>;
}
