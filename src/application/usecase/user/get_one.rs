use crate::{
    application::gateway::repository::user::{GetError, Repo},
    domain::entity::user::{Id, User},
};

use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub user: User,
}

/// Get all users usecase interactor
pub struct GetOne<'r, R> {
    repo: &'r R,
}

impl<'r, R> GetOne<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", GetError::NotFound)]
    NotFound,
    #[error("{}", GetError::Connection)]
    Repo,
}

impl From<GetError> for Error {
    fn from(e: GetError) -> Self {
        match e {
            GetError::Connection => Self::Repo,
            GetError::NotFound => Self::NotFound,
        }
    }
}

impl<'r, R> GetOne<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Get user by ID");
        let user = self.repo.get(req.id)?.into();
        Ok(Response { user })
    }
}
