use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    signup_process::{EmailVerified, Failed, Id, SignupProcess},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Id,
}
pub struct ExtendCompletionTime<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
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
            GetError::IncorrectState => Self::IncorrectState(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for ExtendCompletionTime<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completion extended: {:?}", req);
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<Failed<EmailVerified>> =
            record.try_into().map_err(|err| (err, req.id))?;
        let process = process.recover();
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, process.into())
            .await
            .map_err(|_| Self::Error::NotFound(req.id))?;
        Ok(Self::Response { id: req.id })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn authorize(_: &Self::Request, auth_context: Option<AuthContext>) -> Result<(), AuthError> {
        // admin only
        if let Some(auth_context) = auth_context {
            if auth_context.is_admin() {
                return Ok(());
            }
        }
        Err(AuthError::Unauthorized)
    }
}
