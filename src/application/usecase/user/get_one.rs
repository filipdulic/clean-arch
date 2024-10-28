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
pub struct GetOne<'d, D> {
    db: &'d D,
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

impl<'d, D> Usecase<'d, D> for GetOne<'d, D>
where
    D: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get user by ID");
        let user = self.db.get(req.id)?.into();
        Ok(Self::Response { user })
    }

    fn new(db: &'d D) -> Self {
        Self { db }
    }
}
