use crate::{
    application::gateway::repository::{
        signup_process::{GetError, Repo, SaveError},
        user,
    },
    domain::entity::{
        signup_process::{EmailAdded, Id, SignupProcess, SignupStateEnum},
        user::User,
    },
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub record: user::Record,
}
pub struct Complete<'r1, 'r2, R1, R2> {
    repo: &'r1 R1,
    user_repo: &'r2 R2,
}

impl<'r1, 'r2, R1, R2> Complete<'r1, 'r2, R1, R2> {
    pub fn new(repo: &'r1 R1, user_repo: &'r2 R2) -> Self {
        Self { repo, user_repo }
    }
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
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

impl<'r1, 'r2, R1, R2> Complete<'r1, 'r2, R1, R2>
where
    R1: Repo,
    R2: user::Repo,
{
    /// Create a new user with the given name.
    pub fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        let record = self
            .repo
            .get_latest_state(req.id)
            .map_err(|_| Error::Repo)?;
        if let SignupStateEnum::EmailAdded { .. } = record.state {
            let process: SignupProcess<EmailAdded> = record.into();
            let process = process.complete();
            self.repo
                .save_latest_state(process.clone().into())
                .map_err(|_| Error::NotFound(req.id))?;
            let user: User = process.into();
            self.user_repo
                .save(user.clone().into())
                .map_err(|_| Error::Repo)?;
            Ok(Response {
                record: user.into(),
            })
        } else {
            Err(Error::Repo)
        }
    }
}
