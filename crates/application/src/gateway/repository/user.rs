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
pub trait Repo: Send + Sync {
    type Transaction;
    fn save(
        &self,
        transaction: Option<&mut Self::Transaction>,
        record: Record,
    ) -> impl Future<Output = Result<(), SaveError>>;
    fn get(
        &self,
        transaction: Option<&mut Self::Transaction>,
        id: Id,
    ) -> impl Future<Output = Result<Record, GetError>>;
    fn get_by_username(
        &self,
        transaction: Option<&mut Self::Transaction>,
        username: UserName,
    ) -> impl Future<Output = Result<Record, GetError>>;
    fn get_all(
        &self,
        transaction: Option<&mut Self::Transaction>,
    ) -> impl Future<Output = Result<Vec<Record>, GetAllError>>;
    fn delete(
        &self,
        transaction: Option<&mut Self::Transaction>,
        id: Id,
    ) -> impl Future<Output = Result<(), DeleteError>>;
}
