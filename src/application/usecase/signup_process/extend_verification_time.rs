use crate::{
    application::gateway::repository::signup_process::{GetError, Repo, SaveError},
    domain::entity::signup_process::{Id, SignupProcess, VerificationTimedOut},
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
pub struct ExtendVerificationTime<'r, R> {
    repo: &'r R,
}

impl<'r, R> ExtendVerificationTime<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
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

impl<'r, R> ExtendVerificationTime<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Verification extended: {:?}", req);
        let record = self
            .repo
            .get_latest_state(req.id)
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<VerificationTimedOut> =
            record.try_into().map_err(|err| (err, req.id))?;
        let process = process.extend_verification_time();
        self.repo
            .save_latest_state(process.into())
            .map_err(|_| Error::NotFound(req.id))?;
        Ok(Response { id: req.id })
    }
}
