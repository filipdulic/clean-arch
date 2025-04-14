use crate::{
    gateway::{
        repository::{
            user::{GetAllError, Repo},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};
use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    user::User,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request;

#[derive(Debug, Serialize)]
pub struct Response {
    pub users: Vec<User>,
}

/// Get all users usecase interactor
pub struct GetAll<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
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
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get all users");
        let users = self
            .dependency_provider
            .database()
            .user_repo()
            .get_all(None)
            .await?
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
    fn authorize(_: &Self::Request, auth_context: Option<AuthContext>) -> Result<(), AuthError> {
        // admin only
        if let Some(auth_context) = auth_context {
            if auth_context.is_admin() {
                return Ok(());
            }
        }
        Err(AuthError::Unauthorized)
    }
}
