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
    pub state: Rc<dyn SignupState + 'static>,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Record {}

impl<S: SignupState + Clone + 'static> From<SignupProcess<S>> for Record {
    fn from(process: SignupProcess<S>) -> Self {
        Record {
            id: process.id(),
            chain: process.chain().clone(),
            state: process.state(),
        }
    }
}

impl<S: SignupState + Clone + 'static> TryFrom<Record> for SignupProcess<S> {
    // can an fail if S state is not present in record.chain
    type Error = GetError;
    fn try_from(value: Record) -> Result<Self, GetError> {
        (value.id, value.chain, value.state)
            .try_into()
            .map_err(|_| GetError::NotFound)
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
        if let Some(initialized) = self.state.as_any().downcast_ref::<Initialized>() {
            write!(f, "Initialized: {:?}", initialized)
        } else if let Some(email_added) = self.state.as_any().downcast_ref::<EmailAdded>() {
            write!(f, "EmailAdded: {:?}", email_added)
        } else if let Some(completed) = self.state.as_any().downcast_ref::<Completed>() {
            write!(f, "Completed: {:?}", completed)
        } else {
            unreachable!();
        }
    }
}
