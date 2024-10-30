use std::sync::Arc;

use crate::{
    application::{
        gateway::repository::{
            signup_process::{GetError, Repo, SaveError},
            user,
        },
        usecase::Usecase,
    },
    domain::entity::{
        signup_process::{EmailVerified, Id, SignupProcess, SignupStateEnum},
        user::{Password, User, UserName},
    },
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Response {
    pub record: user::Record,
}
pub struct Complete<D> {
    db: Arc<D>,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
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

impl<D> Usecase<D> for Complete<D>
where
    D: Repo + user::Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        let record = self
            .db
            .get_latest_state(req.id)
            .map_err(|_| Self::Error::Repo)?;
        if let SignupStateEnum::EmailVerified { .. } = record.state {
            let process: SignupProcess<EmailVerified> =
                record.try_into().map_err(|err| (err, req.id))?;
            let username = UserName::new(req.username);
            let password = Password::new(req.password);
            let process = process.complete(username, password);
            let user: User = User::new(
                crate::domain::entity::user::Id::new(req.id),
                process.email(),
                process.username(),
                process.password(),
            );
            // Save User first, then save SignupProcess
            self.db.save(user.clone().into()).map_err(|_| Error::Repo)?;
            // if save user fails, we should not save the signup process
            self.db
                .save_latest_state(process.clone().into())
                .map_err(|_| Self::Error::NotFound(req.id))?;
            Ok(Self::Response {
                record: user.into(),
            })
        } else {
            Err(Self::Error::Repo)
        }
    }

    fn new(db: Arc<D>) -> Self {
        Self { db }
    }
}
