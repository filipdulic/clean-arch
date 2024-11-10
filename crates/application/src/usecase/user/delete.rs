use crate::{
    gateway::{repository::user::DeleteError, UserRepoProvider},
    usecase::{Comitable, Usecase},
};

use ca_domain::entity::user::Id;
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response;

/// Delete area of life by ID usecase interactor
pub struct Delete<'d, D> {
    dependency_provider: &'d D,
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

impl<'d, D> Usecase<'d, D> for Delete<'d, D>
where
    D: UserRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Delete User by ID: {:?}", req);
        self.dependency_provider.user_repo().delete(req.id)?;
        Ok(Self::Response {})
    }

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
}

impl From<Result<Response, Error>> for Comitable<Response, Error> {
    fn from(res: Result<Response, Error>) -> Self {
        match res {
            Ok(res) => Comitable::Commit(Ok(res)),
            Err(err) => Comitable::Rollback(Err(err)),
        }
    }
}
