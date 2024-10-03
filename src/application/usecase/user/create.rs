use crate::{
    application::{
        gateway::repository::user::{Repo, SaveError},
        identifier::{NewId, NewIdError},
        usecase::user::validate::{self, validate_user_properties, UserInvalidity},
    },
    domain::entity::user::{Email, Id, User, UserName},
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub username: String,
    pub email: String,
}

#[derive(Debug)]
pub struct Response {
    pub id: Id,
}

pub struct Create<'r, 'g, R, G> {
    repo: &'r R,
    id_gen: &'g G,
}

impl<'r, 'g, R, G> Create<'r, 'g, R, G> {
    pub fn new(repo: &'r R, id_gen: &'g G) -> Self {
        Self { repo, id_gen }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error(transparent)]
    Invalidity(#[from] UserInvalidity),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl<'r, 'g, R, G> Create<'r, 'g, R, G>
where
    R: Repo,
    G: NewId<Id>,
{
    /// Create a new user with the given name.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Create new user: {:?}", req);
        validate_user_properties(&validate::Request {
            username: &req.username,
            email: &req.email,
        })?;
        let username = UserName::new(req.username);
        let email = Email::new(req.email);
        let id = self.id_gen.new_id().map_err(|err| {
            log::warn!("{}", err);
            Error::NewId
        })?;
        let user = User::new(id, username, email);
        self.repo.save(user)?;
        Ok(Response { id })
    }
}
