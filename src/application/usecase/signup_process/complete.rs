use crate::{
    application::{
        gateway::repository::{
            signup_process::{GetError, Repo, SaveError},
            user,
        },
        usecase::Usecase,
    },
    domain::entity::{
        signup_process::{EmailVerified, Id, SignupProcess},
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

pub struct Complete;

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

impl<D> Usecase<D> for Complete
where
    D: Repo + user::Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(db: &D, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        let record = db.get_latest_state(req.id).map_err(|err| (err, req.id))?;
        let process: SignupProcess<EmailVerified> =
            record.try_into().map_err(|err| (err, req.id))?;
        let username = UserName::new(req.username);
        let password = Password::new(req.password);
        let process = process.complete(username, password);
        db.save_latest_state(process.clone().into())?;
        let user: User = process.into();
        db.save(user.clone().into()).map_err(|_| Error::Repo)?;
        Ok(Response {
            record: user.into(),
        })
    }
}
