use crate::domain::entity::signup_process::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error("SignupProcess not found")]
    NotFound,
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("SignupProcess not found")]
    NotFound,
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub id: Id,
    pub chain: Vec<SignupStateEnum>,
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
            chain: process.chain().clone(),
        }
    }
}

impl<S: SignupStateTrait> From<Record> for SignupProcess<S> {
    fn from(value: Record) -> Self {
        (value.id, value.chain).into()
    }
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn save_latest_state(&self, record: Record) -> Result<(), SaveError>;
    fn get_latest_state(&self, id: Id) -> Result<Record, GetError>;
    fn get_state_chain(&self, id: Id) -> Result<Vec<Record>, GetError>;
    fn delete(&self, id: Id) -> Result<(), DeleteError>;
}
