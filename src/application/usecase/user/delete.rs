use crate::{
    application::gateway::repository::user::{DeleteError, Repo},
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

impl<'r, R> Delete<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
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

impl<'r, R> Delete<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Delete User by ID: {:?}", req);
        self.repo.delete(req.id)?;
        Ok(Response {})
    }
}
