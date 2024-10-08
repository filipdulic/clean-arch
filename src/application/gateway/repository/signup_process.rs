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

#[derive(Clone)]
pub struct Record {
    pub id: Id,
    pub chain: Vec<Rc<dyn SignupState>>,
    pub state: Rc<dyn SignupState>,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Record {}

impl<S: SignupState> From<SignupProcess<S>> for Record {
    fn from(process: SignupProcess<S>) -> Self {
        Record {
            id: process.id(),
            chain: process.chain().clone(),
            state: process.state(),
        }
    }
}

impl<S: SignupState + Clone> From<Record> for SignupProcess<S> {
    fn from(value: Record) -> Self {
        if let Some(state) = value.state.as_any().downcast_ref::<S>() {
            SignupProcess::from_params(value.id, value.chain, Rc::new(state.clone()))
        } else {
            unreachable!()
        }
    }
}

// TODO: make it async
pub trait Repo: Send + Sync {
    fn save(&self, record: Record) -> Result<(), SaveError>;
    fn get(&self, id: Id) -> Result<Record, GetError>;
    fn delete(&self, id: Id) -> Result<(), DeleteError>;
}

impl std::fmt::Debug for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignupProcess")
            .field("id", &self.id)
            .field("state", &self.state)
            .finish()
    }
}
