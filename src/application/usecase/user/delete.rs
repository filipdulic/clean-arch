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
pub struct Delete<'r, R> {
    repo: &'r R,
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

impl<'r, R> Usecase<'r, R> for Delete<'r, R>
where
    R: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Delete User by ID: {:?}", req);
        self.repo.delete(req.id)?;
        Ok(Response {})
    }

    fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}
