use crate::{
    application::{
        gateway::repository::user::{DeleteError, Repo},
        usecase::Usecase,
    },
    domain::entity::user::Id,
};
use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response;

pub struct Delete;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", DeleteError::NotFound)]
    NotFound,
    #[error("{}", DeleteError::Connection)]
    Repo,
}

impl From<DeleteError> for Error {
    fn from(e: DeleteError) -> Self {
        match e {
            DeleteError::NotFound => Self::NotFound,
            DeleteError::Connection => Self::Repo,
        }
    }
}

impl<D: Repo> Usecase<D> for Delete {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(db: &D, req: Request) -> Result<Response, Error> {
        log::debug!("Delete User by ID: {:?}", req);
        db.delete(req.id)?;
        Ok(Response)
    }
}
