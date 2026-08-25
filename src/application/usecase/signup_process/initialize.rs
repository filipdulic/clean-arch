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

pub struct Initialize;

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

impl<D> Usecase<D> for Initialize
where
    D: Repo + NewId<Id>,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    /// TODO: add transaction and outbox pattern to send the verification email.
    fn exec(db: &D, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Initialized: {:?}", req);
        let id = db.new_id().map_err(|_| Error::NewId)?;
        let email = Email::new(req.email);
        let signup_process = SignupProcess::new(id, email);
        db.save_latest_state(signup_process.into())?;
        Ok(Response { id })
    }
}
