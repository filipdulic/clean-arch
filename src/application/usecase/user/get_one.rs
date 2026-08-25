use crate::{
    application::{
        gateway::repository::user::{GetError, Repo},
        usecase::Usecase,
    },
    domain::entity::user::{Id, User},
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub user: User,
}

pub struct GetOne;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", GetError::NotFound)]
    NotFound,
    #[error("{}", GetError::Connection)]
    Repo,
}

impl From<GetError> for Error {
    fn from(e: GetError) -> Self {
        match e {
            GetError::Connection => Self::Repo,
            GetError::NotFound => Self::NotFound,
        }
    }
}

impl<D: Repo> Usecase<D> for GetOne {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(db: &D, req: Request) -> Result<Response, Error> {
        log::debug!("Get user by ID");
        let user = db.get(req.id)?.into();
        Ok(Response { user })
    }
}
