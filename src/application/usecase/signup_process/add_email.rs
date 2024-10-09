use crate::{
    application::gateway::repository::signup_process::{GetError, Repo, SaveError},
    domain::entity::{
        signup_process::{Id, Initialized, SignupProcess},
        user::Email,
    },
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
    pub email: String,
}

#[derive(Debug)]
pub struct Response {
    pub id: Id,
}
pub struct AddEmail<'r, R> {
    repo: &'r R,
}

impl<'r, R> AddEmail<'r, R> {
    pub fn new(repo: &'r R) -> Self {
        Self { repo }
    }
}

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

impl<'r, R> AddEmail<'r, R>
where
    R: Repo,
{
    /// Create a new user with the given name.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Email Added: {:?}", req);
        let email = Email::new(req.email);
        let record = self.repo.get(req.id).map_err(|err| (err, req.id))?;
        // NOTE: error hadnling trick with adding id to error.
        let sp: SignupProcess<Initialized> = record.try_into().map_err(|err| (err, req.id))?;
        let sp = sp.add_email(email);
        self.repo.save(sp.into())?;
        Ok(Response { id: req.id })
    }
}
