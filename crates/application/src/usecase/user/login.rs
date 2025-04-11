use crate::{
    gateway::{
        repository::user::{GetError, Repo, SaveError},
        service::auth::AuthPacker,
        AuthPackerProvider, UserRepoProvider,
    },
    usecase::{Comitable, Usecase},
};
use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    user::{Id, Password, UserName},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub user_id: Id,
    pub token: String,
}

pub struct Login<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("User with username {0} not found")]
    NotFound(UserName),
    #[error("User password or username is invalid")]
    InvalidLogin,
    #[error("{}", SaveError::Connection)]
    Repo,
}

impl From<SaveError> for Error {
    fn from(err: SaveError) -> Self {
        match err {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<(GetError, UserName)> for Error {
    fn from((err, user_name): (GetError, UserName)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(user_name),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for Login<'d, D>
where
    D: UserRepoProvider + AuthPackerProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Login User: {:?}", req.username);
        let user_name = UserName::new(&req.username);
        let password = Password::new(&req.password);
        let record = self
            .dependency_provider
            .user_repo()
            .get_by_username(user_name.clone())
            .await
            .map_err(|err| (err, user_name))?;
        // check password
        if password.ne(record.user.password()) {
            return Err(Error::InvalidLogin);
        }
        let auth_context = AuthContext::new(record.user.id(), record.user.role().clone());
        let token = self
            .dependency_provider
            .auth_packer()
            .pack_auth(auth_context)
            .await;
        Ok(Response {
            user_id: record.user.id(),
            token,
        })
    }

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn authorize(_: &Self::Request, _: Option<AuthContext>) -> Result<(), AuthError> {
        // public signup endpoint, open/no auth
        Ok(())
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
