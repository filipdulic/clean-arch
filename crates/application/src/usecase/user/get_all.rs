use crate::{
    gateway::repository::user::{GetAllError, Repo},
    usecase::Usecase,
};
use ca_domain::entity::user::User;
use thiserror::Error;

#[derive(Debug)]
pub struct Request;

#[derive(Debug)]
pub struct Response {
    pub users: Vec<User>,
}

/// Get all users usecase interactor
pub struct GetAll<'d, D> {
    db: &'d D,
}

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

impl<'d, D> Usecase<'d, D> for GetAll<'d, D>
where
    D: Repo,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get all users");
        let users = self.db.get_all()?.into_iter().map(User::from).collect();
        Ok(Self::Response { users })
    }

    fn new(db: &'d D) -> Self {
        Self { db }
    }
}
