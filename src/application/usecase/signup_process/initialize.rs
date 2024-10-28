use crate::{
    application::{
        gateway::repository::signup_process::{Repo, SaveError},
        identifier::{NewId, NewIdError},
        usecase::Usecase,
    },
    domain::entity::{
        signup_process::{Id, SignupProcess},
        user::Email,
    },
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub email: String,
}

#[derive(Debug)]
pub struct Response {
    pub id: Id,
}
pub struct Initialize<'d, D> {
    // TODO: figure out a way to separete the id generation from the repo.
    // same issue in complete, perhaps a special service or unit of work?
    db: &'d D,
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

impl<'d, D> Usecase<'d, D> for Initialize<'d, D>
where
    D: Repo + NewId<Id>,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    /// Create a new user with the given name.
    /// TODO: add transaction, outbox pattern to send email.
    /// when the user is created, send an email to the user.
    /// with generated token.
    fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Initialized: {:?}", req);
        let id = self.db.new_id().map_err(|_| Error::NewId)?;
        let email = Email::new(req.email);
        let signup_process = SignupProcess::new(id, email);
        self.db.save_latest_state(signup_process.into())?;
        Ok(Response { id })
    }
    fn new(db: &'d D) -> Self {
        Self { db }
    }
}
