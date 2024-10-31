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
    pub state: SignupStateEnum,
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
        }
    }
}

impl<S: SignupStateTrait + Clone> TryFrom<Record> for SignupProcess<S> {
    type Error = GetError;
    fn try_from(value: Record) -> Result<Self, Self::Error> {
        (value.id, value.state)
            .try_into()
            .map_err(|_| GetError::NotFound)
    }
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn save_latest_state(&self, record: Record) -> Result<(), SaveError>;
    fn get_latest_state(&self, id: Id) -> Result<Record, GetError>;
    fn get_state_chain(&self, id: Id) -> Result<Vec<Record>, GetError>;
    fn delete(&self, id: Id) -> Result<(), DeleteError>;
}
