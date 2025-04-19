use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            token::{GenError as TokenRepoError, Repo as TokenRepo},
            Database,
        },
        service::email::{EmailAddress, EmailServiceError, EmailVerificationService},
        DatabaseProvider, EmailVerificationServiceProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    signup_process::{Error as SignupProcessError, Id, Initialized, SignupProcess},
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
pub struct SendVerificationEmail<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
    #[error("SignupProcess Repo error")]
    Repo,
    #[error("Token Repo error: {0}")]
    TokenRepoError(#[from] TokenRepoError),
    #[error("Email Service error: {0}")]
    EmailServiceError(#[from] EmailServiceError),
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

impl From<SaveError> for Error {
    fn from(_: SaveError) -> Self {
        Self::Repo
    }
}

impl<'d, D> Usecase<'d, D> for SendVerificationEmail<'d, D>
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess SendVerificationEmail ID: {:?}", req);
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<Initialized> = record.try_into().map_err(|err| (err, req.id))?;
        let token = match self
            .dependency_provider
            .database()
            .token_repo()
            .gen(None, process.state().email.as_ref())
            .await
        {
            Ok(record) => record.token,
            Err(err) => {
                log::error!("Token Repo error: {:?}", err);
                let process = process.fail(SignupProcessError::TokenGenrationFailed);
                self.dependency_provider
                    .database()
                    .signup_process_repo()
                    .save_latest_state(None, process.into())
                    .await?;
                return Err(err.into());
            }
        };
        if let Err(err) = self
            .dependency_provider
            .email_verification_service()
            .send_verification_email(
                EmailAddress::new(process.state().email.as_ref()),
                token.as_str(),
            )
            .await
        {
            log::error!("Email Service error: {:?}", err);
            let process = process.fail(SignupProcessError::VerificationEmailSendError);
            self.dependency_provider
                .database()
                .signup_process_repo()
                .save_latest_state(None, process.into())
                .await?;
            return Err(err.into());
        }
        let process = process.send_verification_email();
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, process.into())
            .await?;
        Ok(Response { id: req.id })
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
