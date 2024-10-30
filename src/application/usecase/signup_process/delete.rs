use std::sync::Arc;

use crate::{
    application::{
        gateway::repository::signup_process::{GetError, Repo, SaveError},
        usecase::Usecase,
    },
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
pub struct Delete<D> {
    db: Arc<D>,
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

impl<D> Usecase<D> for Delete<D>
where
    D: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess scheduled for deletion: {:?}", req);
        let record = self
            .db
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

        self.db
            .save_latest_state(process.into())
            .map_err(|_| Self::Error::NotFound(req.id))?;
        Ok(Self::Response { id: req.id })
    }
    fn new(db: Arc<D>) -> Self {
        Self { db }
    }
}
