use crate::{
    gateway::{
        repository::{signup_process::SaveError, token::GenError as TokenRepoError},
        service::email::EmailAddress,
        EmailVerificationServiceProvider, SignupProcessIdGenProvider, SignupProcessRepoProvider,
        TokenRepoProvider,
    },
    identifier::NewIdError,
    usecase::Usecase,
};

use ca_domain::entity::{
    signup_process::{Id, SignupProcess},
    user::Email,
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
    dependency_provider: &'d D,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error("Token Repo error: {0}")]
    TokenRepoError(#[from] TokenRepoError),
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
    D: SignupProcessIdGenProvider
        + SignupProcessRepoProvider
        + EmailVerificationServiceProvider
        + TokenRepoProvider,
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
        let id = self
            .dependency_provider
            .signup_process_id_gen()
            .new_id()
            .map_err(|_| Error::NewId)?;
        let email = Email::new(req.email);
        let signup_process = SignupProcess::new(id, email.clone());
        self.dependency_provider
            .signup_process_repo()
            .save_latest_state(signup_process.into())?;
        let token = self
            .dependency_provider
            .token_repo()
            .gen(email.as_ref())?
            .token;
        self.dependency_provider
            .email_verification_service()
            .send_verification_email(EmailAddress::new(email.as_ref()), token.as_str())
            .unwrap();
        Ok(Response { id })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
}
