use std::future::Future;

use ca_domain::entity::signup_process::*;
use chrono::{DateTime, Utc};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum GetError {
    #[error("SignupProcess not found")]
    NotFound,
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Error, Serialize)]
pub enum SaveError {
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Error, Serialize)]
pub enum DeleteError {
    #[error("SignupProcess not found")]
    NotFound,
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub id: Id,
    pub state: SignupStateEnum,
    pub entered_at: DateTime<Utc>,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Record {}

impl<S: SignupStateTrait> From<SignupProcess<S>> for Record {
    fn from(process: SignupProcess<S>) -> Self {
        Record {
            id: process.id(),
            state: process.state().clone().into(),
            entered_at: process.entered_at(),
        }
    }
}

impl<S: SignupStateTrait + Clone> TryFrom<Record> for SignupProcess<S> {
    type Error = GetError;
    fn try_from(value: Record) -> Result<Self, Self::Error> {
        (value.id, value.state, value.entered_at)
            .try_into()
            .map_err(|_| GetError::NotFound)
    }
}

pub trait Repo: Send + Sync {
    fn save_latest_state(&self, record: Record) -> impl Future<Output = Result<(), SaveError>>;
    fn get_latest_state(&self, id: Id) -> impl Future<Output = Result<Record, GetError>>;
    fn get_state_chain(&self, id: Id) -> impl Future<Output = Result<Vec<Record>, GetError>>;
    fn delete(&self, id: Id) -> impl Future<Output = Result<(), DeleteError>>;
}
