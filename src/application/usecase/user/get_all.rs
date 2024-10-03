use crate::{
    application::gateway::repository::user::{GetAllError, Repo},
    domain::entity::user::User,
};

use std::fmt::Debug;
use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response {
    pub users: Vec<User>,
}

/// Get all users usecase interactor
pub struct GetAll<'r, R> {
    repo: &'r R,
}

impl<'r, R> GetAll<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", GetAllError::Connection)]
    Repo,
}

impl From<GetAllError> for Error {
    fn from(e: GetAllError) -> Self {
        match e {
            GetAllError::Connection => Self::Repo,
        }
    }
}

impl<'r, R> GetAll<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, _: Request) -> Result<Response, Error> {
        log::debug!("Get all users");
        let users = self.repo.get_all()?.into_iter().collect();
        Ok(Response { users })
    }
}
