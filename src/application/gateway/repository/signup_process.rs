use std::rc::Rc;

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

pub struct Record {
    id: Id,
    chain: Vec<Rc<dyn SignupState>>,
    state: Rc<dyn SignupState>,
}

impl<S: SignupState> From<SignupProcess<S>> for Record {
    fn from(process: SignupProcess<S>) -> Self {
        Self {
            id: process.id,
            chain: process.chain,
            state: process.state,
        }
    }
}

impl<S: SignupState + Clone> TryFrom<Record> for SignupProcess<S> {
    type Error = ();

    fn try_from(record: Record) -> Result<Self, Self::Error> {
        if let Some(state) = record.state.as_any().downcast_ref::<S>() {
            return Ok(Self {
                id: record.id,
                chain: record.chain,
                state: Rc::new(state.clone()),
            });
        }
        Err(())
    }
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn save(&self, record: impl Into<Record>) -> Result<(), SaveError>;
    fn get(&self, id: impl Into<Id>) -> Result<Record, GetError>;
    fn delete(&self, id: impl Into<Id>) -> Result<(), DeleteError>;
}
