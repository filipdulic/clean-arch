use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            token::{Repo as TokenRepo, VerifyError as TokenRepoError},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    signup_process::{Id, SignupProcess, VerificationEmailSent},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    pub id: Id,
    #[validate(length(min = 1, max = 255))]
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
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("Token Repo error: {0}")]
    TokenRepoError(#[from] TokenRepoError),
    #[error(transparent)]
    TokenInvalidity(#[from] validator::ValidationErrors),
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

impl<'d, D> Usecase<'d, D> for VerifyEmail<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    /// Create a new user with the given name.
    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Email Verification: {:?}", req);
        // Validate the request
        req.validate()?;
        // Begin transaction
        let mut transaction = self
            .dependency_provider
            .database()
            .begin_transaction()
            .await;
        // Load record
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(Some(&mut transaction), req.id)
            .await
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<VerificationEmailSent> =
            record.try_into().map_err(|err| (err, req.id))?;
        // Verify the token
        if let Err(err) = self
            .dependency_provider
            .database()
            .token_repo()
            .verify(
                Some(&mut transaction),
                process.state().email.as_ref(),
                &req.token,
            )
            .await
        {
            log::error!("Token Repo error: {:?}", err);
            if let TokenRepoError::TokenExpired = err {
                let process =
                    process.fail(ca_domain::entity::signup_process::Error::VerificationTimedOut);
                self.dependency_provider
                    .database()
                    .signup_process_repo()
                    .save_latest_state(Some(&mut transaction), process.into())
                    .await?;
            }
            self.dependency_provider
                .database()
                .commit_transaction(transaction)
                .await
                .map_err(|_| SaveError::Connection)?;
            return Err(err.into());
        };
        // Update the process state
        let process = process.verify_email();
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(Some(&mut transaction), process.into())
            .await?;
        self.dependency_provider
            .database()
            .commit_transaction(transaction)
            .await
            .map_err(|_| SaveError::Connection)?;
        Ok(Self::Response { id: req.id })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn authorize(_: &Self::Request, _: Option<AuthContext>) -> Result<(), AuthError> {
        // public signup endpoint, open/no auth
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{gateway::mock::MockDependencyProvider, usecase::tests::fixtures::*};

    use super::*;

    #[rstest]
    async fn test_fails_verify_token_min_lenght(dependency_provider: MockDependencyProvider) {
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        let id = Id::from(uuid::Uuid::new_v4());
        let req = super::Request {
            id,
            token: "".to_string(),
        };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("token: Validation error: length"));
    }
}
