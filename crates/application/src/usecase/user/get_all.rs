use crate::{
    gateway::{repository::user::GetAllError, UserRepoProvider},
    usecase::Usecase,
};
use ca_domain::entity::user::User;
use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response {
    pub users: Vec<User>,
}

/// Get all users usecase interactor
pub struct GetAll<'d, D> {
    dependency_provider: &'d D,
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

impl<'d, D> Usecase<'d, D> for GetAll<'d, D>
where
    D: UserRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get all users");
        let users = self
            .dependency_provider
            .user_repo()
            .get_all()?
            .into_iter()
            .map(User::from)
            .collect();
        Ok(Self::Response { users })
    }

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
}
