use crate::{
    gateway::{repository::user::GetError, UserRepoProvider},
    usecase::{Comitable, Usecase},
};
use ca_domain::entity::user::{Id, User};

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
    dependency_provider: &'d D,
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
    D: UserRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get user by ID");
        let user = self.dependency_provider.user_repo().get(req.id)?.into();
        Ok(Self::Response { user })
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
