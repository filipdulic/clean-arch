use crate::{
    gateway::{
        repository::signup_process::{GetError, Record},
        SignupProcessRepoProvider,
    },
    usecase::{Comitable, Usecase},
};

use ca_domain::entity::signup_process::Id;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub state_chain: Vec<Record>,
}

pub struct GetStateChain<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("{}", GetError::Connection)]
    Repo,
}

impl From<(GetError, Id)> for Error {
    fn from((err, id): (GetError, Id)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for GetStateChain<'d, D>
where
    D: SignupProcessRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Get signup process state chain");
        let state_chain = self
            .dependency_provider
            .signup_process_repo()
            .get_state_chain(req.id)
            .map_err(|err| (err, req.id))?;
        Ok(Self::Response { state_chain })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
}

impl From<Result<Response, Error>> for Comitable<Response, Error> {
    fn from(res: Result<Response, Error>) -> Self {
        match res {
            Ok(res) => Comitable::Commit(Ok(res)),
            Err(err) => Comitable::Rollback(Err(err)),
        }
    }
}
