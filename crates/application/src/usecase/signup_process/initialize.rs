use crate::{
    gateway::{
        database::{
            identifier::{NewId, NewIdError},
            signup_process::{Repo, SaveError},
            token::GenError as TokenRepoError,
            Database,
        },
        service::email::EmailServiceError,
        DatabaseProvider,
    },
    usecase::Usecase,
};
use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    signup_process::{Id, SignupProcess},
    user::Email,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Id,
}
pub struct Initialize<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error("Token Repo error: {0}")]
    TokenRepoError(#[from] TokenRepoError),
    #[error("Email Service error: {0}")]
    EmailServiceError(#[from] EmailServiceError),
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
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    /// Create a new user with the given name.
    /// TODO: add transaction, outbox pattern to send email.
    /// when the user is created, send an email to the user.
    /// with generated token.
    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Initialized: {:?}", req);
        let id = self
            .dependency_provider
            .database()
            .signuo_id_gen()
            .new_id()
            .await
            .map_err(|_| Error::NewId)?;
        let email = Email::new(&req.email);
        let signup_process = SignupProcess::new(id, email);
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, signup_process.into())
            .await?;
        Ok(Response { id })
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
    use super::*;
    use crate::gateway::database::mock::MockDependencyProvider;

    #[tokio::test]
    async fn test_initialize_fails_signup_id_gen() {
        let mut db_provider = MockDependencyProvider::new();
        db_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(|| Box::pin(async { Err(NewIdError) }));
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &db_provider,
        );
        let req = super::Request {
            email: "email@test.com".to_string(),
        };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::NewId);
    }
}
