use crate::{
    application::{
        gateway::repository::user::{GetError, Repo, SaveError},
        usecase::user::validate::{self, validate_user_properties, UserInvalidity},
    },
    domain::entity::user::{Email, Id, User, UserName},
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
    pub username: String,
    pub email: String,
}

pub type Response = ();

pub struct Update<'r, R> {
    repo: &'r R,
}

impl<'r, R> Update<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("User {0} not found")]
    NotFound(Id),
    #[error(transparent)]
    Invalidity(#[from] UserInvalidity),
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

impl<'r, R> Update<'r, R>
where
    R: Repo,
{
    /// Update a area of life.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Update User: {:?}", req);
        validate_user_properties(&validate::Request {
            username: &req.username,
            email: &req.email,
        })?;
        let username = UserName::new(req.username);
        let email = Email::new(req.email);
        let user = User::new(req.id, username, email);
        let _ = self.repo.get(req.id).map_err(|err| (err, req.id))?;
        self.repo.save(user.into())?;
        Ok(())
    }
}
