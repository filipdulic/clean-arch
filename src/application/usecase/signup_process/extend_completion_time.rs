use crate::{
    application::{
        gateway::repository::signup_process::{GetError, Repo, SaveError},
        usecase::Usecase,
    },
    domain::entity::signup_process::{CompletionTimedOut, Id, SignupProcess},
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

pub struct ExtendCompletionTime;

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

impl<D: Repo> Usecase<D> for ExtendCompletionTime {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(db: &D, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Completion extended: {:?}", req);
        let record = db.get_latest_state(req.id).map_err(|err| (err, req.id))?;
        let process: SignupProcess<CompletionTimedOut> =
            record.try_into().map_err(|err| (err, req.id))?;
        let process = process.extend_completion_time();
        db.save_latest_state(process.into())?;
        Ok(Response { id: req.id })
    }
}
