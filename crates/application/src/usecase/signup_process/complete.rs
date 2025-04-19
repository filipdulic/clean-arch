use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            user::{self, Repo as UserRepo},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
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
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    pub id: Id,
    #[validate(length(min = 1, max = 30))]
    pub username: String,
    #[validate(length(min = 5, max = 60))]
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
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
    #[error("SignupProcess completion timed out")]
    CompletionTimedOut,
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
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
            GetError::IncorrectState => Self::IncorrectState(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for Complete<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        // Validate the request
        req.validate()?;
        let transaction = self
            .dependency_provider
            .database()
            .begin_transaction()
            .await;
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(None, req.id)
            .await
            .map_err(|_| Self::Error::Repo)?;
        let process: SignupProcess<EmailVerified> =
            record.try_into().map_err(|_| Self::Error::Repo)?;
        if Utc::now() - Duration::days(1) > process.entered_at() {
            let process =
                process.fail(ca_domain::entity::signup_process::Error::CompletionTimedOut);
            self.dependency_provider
                .database()
                .signup_process_repo()
                .save_latest_state(None, process.into())
                .await?;
            self.dependency_provider
                .database()
                .commit_transaction(transaction)
                .await
                .map_err(|_| Error::Repo)?;
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
            .database()
            .user_repo()
            .save(None, user.clone().into())
            .await
            .map_err(|_| Error::Repo)?;
        // if save user fails, we should not save the signup process
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, process.clone().into())
            .await
            .map_err(|_| Self::Error::NotFound(req.id))?;
        self.dependency_provider
            .database()
            .commit_transaction(transaction)
            .await
            .map_err(|_| Error::Repo)?;
        Ok(Self::Response {
            record: user.into(),
        })
    }

    fn new(db: &'d D) -> Self {
        Self {
            dependency_provider: db,
        }
    }
    fn authorize(_: &Self::Request, _: Option<AuthContext>) -> Result<(), AuthError> {
        // public signup endpoint, open/no auth
        Ok(())
    }
}
