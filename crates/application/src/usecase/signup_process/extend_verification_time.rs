use crate::{
    gateway::{
        repository::{
            signup_process::{GetError, Repo, SaveError},
            token::Repo as TokenRepo,
        },
        SignupProcessRepoProvider, TokenRepoProvider,
    },
    usecase::{Comitable, Usecase},
};

use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    signup_process::{Failed, Id, SignupProcess, VerificationEmailSent},
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
pub struct ExtendVerificationTime<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("Token Extension Error {0}")]
    TokenRepoError(#[from] super::super::super::gateway::repository::token::ExtendError),
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

impl<'d, D> Usecase<'d, D> for ExtendVerificationTime<'d, D>
where
    D: SignupProcessRepoProvider + TokenRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Verification extended: {:?}", req);
        let record = self
            .dependency_provider
            .signup_process_repo()
            .get_latest_state(req.id)
            .await
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<Failed<VerificationEmailSent>> =
            record.try_into().map_err(|err| (err, req.id))?;
        // update token
        let process = process.recover();
        self.dependency_provider
            .token_repo()
            .extend(process.state().email.as_ref())
            .await?;
        self.dependency_provider
            .signup_process_repo()
            .save_latest_state(process.into())
            .await
            .map_err(|_| Self::Error::NotFound(req.id))?;
        Ok(Self::Response { id: req.id })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn is_transactional() -> bool {
        true
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

impl From<Result<Response, Error>> for Comitable<Response, Error> {
    fn from(res: Result<Response, Error>) -> Self {
        match res {
            Ok(res) => Comitable::Commit(Ok(res)),
            Err(err) => Comitable::Rollback(Err(err)),
        }
    }
}
