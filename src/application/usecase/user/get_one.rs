use crate::{
    application::{
        gateway::repository::user::{GetError, Repo},
        usecase::Usecase,
    },
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

impl<'r, R> Usecase<'r, R> for GetOne<'r, R>
where
    R: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get user by ID");
        let user = self.repo.get(req.id)?.into();
        Ok(Self::Response { user })
    }

    fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}
