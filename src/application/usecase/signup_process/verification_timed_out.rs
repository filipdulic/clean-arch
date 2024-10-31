use crate::{
    application::{
        gateway::repository::signup_process::{GetError, Repo, SaveError},
        usecase::Usecase,
    },
    domain::entity::signup_process::{Id, Initialized, SignupProcess},
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub id: Id,
}
pub struct VerificationTimedOut<'d, D> {
    db: &'d D,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
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

impl From<(GetError, Id)> for Error {
    fn from((err, id): (GetError, Id)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for VerificationTimedOut<'d, D>
where
    D: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Verification timed out: {:?}", req);
        let record = self
            .db
            .get_latest_state(req.id)
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<Initialized> = record.try_into().map_err(|err| (err, req.id))?;
        let process = process.verification_timed_out();
        self.db
            .save_latest_state(process.into())
            .map_err(|_| Error::NotFound(req.id))?;
        Ok(Self::Response { id: req.id })
    }
    fn new(db: &'d D) -> Self {
        Self { db }
    }
}
