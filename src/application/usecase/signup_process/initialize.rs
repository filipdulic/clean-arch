use crate::{
    application::{
        gateway::repository::signup_process::{Repo, SaveError},
        identifier::{NewId, NewIdError},
    },
    domain::entity::{
        signup_process::{Id, SignupProcess},
        user::UserName,
    },
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub username: String,
}

#[derive(Debug)]
pub struct Response {
    pub id: Id,
}
pub struct Initialize<'r, 'g, R, G> {
    repo: &'r R,
    id_gen: &'g G,
}

impl<'r, 'g, R, G> Initialize<'r, 'g, R, G> {
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
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl<'r, 'g, R, G> Initialize<'r, 'g, R, G>
where
    R: Repo,
    G: NewId<Id>,
{
    /// Create a new user with the given name.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Initialized: {:?}", req);
        let id = self.id_gen.new_id().map_err(|_| Error::NewId)?;
        let username = UserName::new(req.username);
        let signup_process = SignupProcess::new(id, username);
        self.repo.save(signup_process.into())?;
        Ok(Response { id })
    }
}
