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
pub struct Complete<'d, D> {
    db: &'d D,
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

impl<'d, D> Usecase<'d, D> for Complete<'d, D>
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
            self.db
                .save_latest_state(process.clone().into())
                .map_err(|_| Self::Error::NotFound(req.id))?;
            let user: User = User::new(
                crate::domain::entity::user::Id::new(req.id),
                process.email(),
                process.username(),
                process.password(),
            );
            self.db.save(user.clone().into()).map_err(|_| Error::Repo)?;
            Ok(Self::Response {
                record: user.into(),
            })
        } else {
            Err(Self::Error::Repo)
        }
    }

    fn new(db: &'d D) -> Self {
        Self { db }
    }
}
