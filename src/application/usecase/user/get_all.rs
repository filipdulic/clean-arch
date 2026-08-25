use crate::{
    application::{
        gateway::repository::user::{GetAllError, Repo},
        usecase::Usecase,
    },
    domain::entity::user::User,
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response {
    pub users: Vec<User>,
}

pub struct GetAll;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", GetAllError::Connection)]
    Repo,
}

impl From<GetAllError> for Error {
    fn from(e: GetAllError) -> Self {
        match e {
            GetAllError::Connection => Self::Repo,
        }
    }
}

impl<D: Repo> Usecase<D> for GetAll {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(db: &D, _: Request) -> Result<Response, Error> {
        log::debug!("Get all users");
        let users = db.get_all()?.into_iter().map(User::from).collect();
        Ok(Response { users })
    }
}
