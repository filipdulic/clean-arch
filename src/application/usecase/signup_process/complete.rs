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
pub struct Complete<'r, R> {
    // TODO: figure out unit of work and transaction operations on db trait I guess?
    repo: &'r R,
    user_repo: &'r R,
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

impl<'r, R> Usecase<'r, R> for Complete<'r, R>
where
    R: Repo + user::Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        let record = self
            .repo
            .get_latest_state(req.id)
            .map_err(|_| Error::Repo)?;
        if let SignupStateEnum::EmailVerified { .. } = record.state {
            let process: SignupProcess<EmailVerified> =
                record.try_into().map_err(|err| (err, req.id))?;
            let username = UserName::new(req.username);
            let password = Password::new(req.password);
            let process = process.complete(username, password);
            self.repo
                .save_latest_state(process.clone().into())
                .map_err(|_| Error::NotFound(req.id))?;
            let user: User = User::new(
                crate::domain::entity::user::Id::new(req.id),
                process.email(),
                process.username(),
                process.password(),
            );
            self.user_repo
                .save(user.clone().into())
                .map_err(|_| Error::Repo)?;
            Ok(Response {
                record: user.into(),
            })
        } else {
            Err(Error::Repo)
        }
    }

    fn new(repo: &'r R) -> Self {
        Self {
            repo,
            user_repo: repo,
        }
    }
}
