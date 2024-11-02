use crate::{
    gateway::repository::signup_process::{GetError, Record, Repo},
    usecase::Usecase,
};

use ca_domain::entity::signup_process::Id;

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub state_chain: Vec<Record>,
}

pub struct GetStateChain<'d, D> {
    db: &'d D,
}

#[derive(Debug, Error)]
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
    D: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Get signup process state chain");
        let state_chain = self
            .db
            .get_state_chain(req.id)
            .map_err(|err| (err, req.id))?;
        Ok(Self::Response { state_chain })
    }
    fn new(db: &'d D) -> Self {
        Self { db }
    }
}
