use std::sync::Arc;

use crate::{
    application::{
        gateway::repository::user::{DeleteError, Repo},
        usecase::Usecase,
    },
    domain::entity::user::Id,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response;

/// Delete area of life by ID usecase interactor
pub struct Delete<D> {
    db: Arc<D>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", DeleteError::NotFound)]
    NotFound,
    #[error("{}", DeleteError::Connection)]
    Repo,
}

impl From<DeleteError> for Error {
    fn from(e: DeleteError) -> Self {
        match e {
            DeleteError::NotFound => Self::NotFound,
            DeleteError::Connection => Self::Repo,
        }
    }
}

impl<D> Usecase<D> for Delete<D>
where
    D: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Delete User by ID: {:?}", req);
        self.db.delete(req.id)?;
        Ok(Self::Response {})
    }

    fn new(db: Arc<D>) -> Self {
        Self { db }
    }
}
