use crate::{
    application::gateway::repository::signup_process::{GetError, Repo, SaveError},
    domain::entity::signup_process::{
        CompletionTimedOut, Id, SignupProcess, SignupStateEnum, VerificationTimedOut,
    },
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
pub struct Delete<'r, R> {
    repo: &'r R,
}

impl<'r, R> Delete<'r, R> {
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

impl<'r, R> Delete<'r, R>
where
    R: Repo,
{
    /// Create a new user with the given name.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess scheduled for deletion: {:?}", req);
        let record = self
            .repo
            .get_latest_state(req.id)
            .map_err(|err| (err, req.id))?;
        let process = match record.state {
            SignupStateEnum::VerificationTimedOut { .. } => {
                match SignupProcess::<VerificationTimedOut>::try_from(record) {
                    Ok(process) => process.delete(),
                    Err(_) => return Err((GetError::NotFound, req.id).into()),
                }
            }
            SignupStateEnum::CompletionTimedOut { .. } => {
                match SignupProcess::<CompletionTimedOut>::try_from(record) {
                    Ok(process) => process.delete(),
                    Err(_) => return Err((GetError::NotFound, req.id).into()),
                }
            }
            _ => return Err((GetError::NotFound, req.id).into()),
        };

        self.repo
            .save_latest_state(process.into())
            .map_err(|_| Error::NotFound(req.id))?;
        Ok(Response { id: req.id })
    }
}
