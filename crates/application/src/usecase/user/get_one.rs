use crate::{
    gateway::{
        database::{
            user::{GetError, Repo},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};
use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    user::{Id, User},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub user: User,
}

/// Get all users usecase interactor
pub struct GetOne<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
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
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get user by ID");
        let user = self
            .dependency_provider
            .database()
            .user_repo()
            .get(None, req.id)
            .await?
            .into();
        Ok(Self::Response { user })
    }

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }

    fn authorize(req: &Self::Request, auth_context: Option<AuthContext>) -> Result<(), AuthError> {
        // owner and admin only
        if let Some(auth_context) = auth_context {
            // admin allowed
            if auth_context.is_admin() {
                return Ok(());
            } else {
                // if requested id is same as auth_context id
                if &req.id == auth_context.user_id() {
                    return Ok(());
                }
            }
        }
        Err(AuthError::Unauthorized)
    }
}
