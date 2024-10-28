use crate::{
    application::{
        gateway::repository::user::{GetAllError, Repo},
        usecase::Usecase,
    },
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

impl<'r, R> Usecase<'r, R> for GetAll<'r, R>
where
    R: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get all users");
        let users = self.repo.get_all()?.into_iter().map(User::from).collect();
        Ok(Response { users })
    }

    fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}
