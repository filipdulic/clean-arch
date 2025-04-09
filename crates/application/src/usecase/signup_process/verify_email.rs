use crate::{
    gateway::{
        repository::{
            signup_process::{GetError, SaveError},
            token::VerifyError as TokenRepoError,
        },
        SignupProcessRepoProvider, TokenRepoProvider,
    },
    usecase::{Comitable, Usecase},
};

use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    signup_process::{Id, SignupProcess, VerificationEmailSent},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Id,
}
pub struct VerifyEmail<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("Token Repo error: {0}")]
    TokenRepoError(#[from] TokenRepoError),
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

impl<'d, D> Usecase<'d, D> for VerifyEmail<'d, D>
where
    D: SignupProcessRepoProvider + TokenRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    /// Create a new user with the given name.
    fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Email Verification: {:?}", req);
        // Load record
        let record = self
            .dependency_provider
            .signup_process_repo()
            .get_latest_state(req.id)
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<VerificationEmailSent> =
            record.try_into().map_err(|err| (err, req.id))?;
        // Verify the token
        if let Err(err) = self
            .dependency_provider
            .token_repo()
            .verify(process.state().email.as_ref(), &req.token)
        {
            log::error!("Token Repo error: {:?}", err);
            if let TokenRepoError::TokenExpired = err {
                let process =
                    process.fail(ca_domain::entity::signup_process::Error::VerificationTimedOut);
                self.dependency_provider
                    .signup_process_repo()
                    .save_latest_state(process.into())?;
            }
            return Err(err.into());
        };
        // Update the process state
        let process = process.verify_email();
        self.dependency_provider
            .signup_process_repo()
            .save_latest_state(process.into())?;
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
    fn authorize(_: &Self::Request, _: Option<AuthContext>) -> Result<(), AuthError> {
        // public signup endpoint, open/no auth
        Ok(())
    }
}

impl From<Result<Response, Error>> for Comitable<Response, Error> {
    fn from(res: Result<Response, Error>) -> Self {
        match res {
            Ok(res) => Comitable::Commit(Ok(res)),
            Err(err) => match err {
                Error::TokenRepoError(TokenRepoError::TokenExpired) => Comitable::Commit(Err(err)),
                _ => Comitable::Rollback(Err(err)),
            },
        }
    }
}
