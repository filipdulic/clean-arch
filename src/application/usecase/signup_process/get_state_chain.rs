use crate::{
    application::gateway::repository::signup_process::{GetError, Record, Repo},
    domain::entity::signup_process::Id,
};

use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub state_chain: Vec<Record>,
}

/// Get signup process state chain
pub struct GetStateChain<'r, R> {
    repo: &'r R,
}

impl<'r, R> GetStateChain<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("{}", GetError::Connection)]
    Repo,
}

impl From<(GetError, Id)> for Error {
    fn from((err, id): (GetError, Id)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'r, R> GetStateChain<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Get signup process state chain");
        let state_chain = self
            .repo
            .get_state_chain(req.id)
            .map_err(|err| (err, req.id))?;
        Ok(Response { state_chain })
    }
}
