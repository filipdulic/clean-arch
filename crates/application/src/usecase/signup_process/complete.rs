use crate::{
    gateway::{
        repository::{
            signup_process::{GetError, SaveError},
            user,
        },
        SignupProcessRepoProvider, UserRepoProvider,
    },
    usecase::{Comitable, Usecase},
};

use ca_domain::{
    entity::{
        auth_context::{AuthContext, AuthError},
        signup_process::{EmailVerified, Id, SignupProcess},
        user::{Password, User, UserName},
    },
    value_object::Role,
};

use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub record: user::Record,
}
pub struct Complete<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess completion timed out")]
    CompletionTimedOut,
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<(GetError, Id)> for Error {
    fn from((err, id): (GetError, Id)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for Complete<'d, D>
where
    D: SignupProcessRepoProvider + UserRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        let record = self
            .dependency_provider
            .signup_process_repo()
            .get_latest_state(req.id)
            .map_err(|_| Self::Error::Repo)?;
        let process: SignupProcess<EmailVerified> =
            record.try_into().map_err(|_| Self::Error::Repo)?;
        if Utc::now() - Duration::days(1) > process.entered_at() {
            let process =
                process.fail(ca_domain::entity::signup_process::Error::CompletionTimedOut);
            self.dependency_provider
                .signup_process_repo()
                .save_latest_state(process.into())?;
            return Err(Self::Error::CompletionTimedOut);
        }
        let username = UserName::new(req.username);
        let password = Password::new(req.password);
        let process = process.complete(username, password);
        let user: User = User::new(
            ca_domain::entity::user::Id::new(req.id),
            Role::User,
            process.email(),
            process.username(),
            process.password(),
        );
        // Save User first, then save SignupProcess
        self.dependency_provider
            .user_repo()
            .save(user.clone().into())
            .map_err(|_| Error::Repo)?;
        // if save user fails, we should not save the signup process
        self.dependency_provider
            .signup_process_repo()
            .save_latest_state(process.clone().into())
            .map_err(|_| Self::Error::NotFound(req.id))?;
        Ok(Self::Response {
            record: user.into(),
        })
    }

    fn new(db: &'d D) -> Self {
        Self {
            dependency_provider: db,
        }
    }
    fn is_transactional() -> bool {
        true
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
            Err(err) => match err {
                Error::CompletionTimedOut => Comitable::Commit(Err(Error::CompletionTimedOut)),
                _ => Comitable::Rollback(Err(err)),
            },
        }
    }
}
