use crate::{
    application::gateway::repository::signup_process::{GetError, Repo, SaveError},
    domain::entity::signup_process::{EmailVerified, Id, SignupProcess},
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
pub struct CompletionTimedOut<'r, R> {
    repo: &'r R,
}

impl<'r, R> CompletionTimedOut<'r, R> {
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

impl<'r, R> CompletionTimedOut<'r, R>
where
    R: Repo,
{
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess verification timed out: {:?}", req);
        let record = self
            .repo
            .get_latest_state(req.id)
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<EmailVerified> =
            record.try_into().map_err(|err| (err, req.id))?;
        let process = process.completion_timed_out();
        self.repo
            .save_latest_state(process.into())
            .map_err(|_| Error::NotFound(req.id))?;
        Ok(Response { id: req.id })
    }
}
