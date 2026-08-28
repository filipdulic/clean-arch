use std::sync::Arc;

use crate::auth::{Authorized, Public};
use crate::{
    gateway::{
        DatabaseProvider,
        database::{
            Database,
            identifier::{NewId, NewIdError},
            signup_process::{Repo, SaveError},
        },
    },
    usecase::Usecase,
};
use ca_domain::entity::{
    signup_process::{Id, SignupProcess},
    user::Email,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    #[validate(email, length(min = 5, max = 254))]
    pub email: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Id,
}
pub struct Initialize<D> {
    dependency_provider: Arc<D>,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("{}", NewIdError)]
    NewId,
    #[error(transparent)]
    EmailInvalidity(#[from] validator::ValidationErrors),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}
#[async_trait::async_trait]
impl<D> Usecase<D> for Initialize<D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Auth = Public;
    /// Create a new user with the given name.
    /// TODO: add transaction, outbox pattern to send email.
    /// when the user is created, send an email to the user.
    /// with generated token.
    async fn exec(&self, req: Authorized<Self::Request>) -> Result<Response, Error> {
        let req = req.into_request();
        log::debug!("SignupProcess Initialized: {:?}", req);
        // validate email
        req.validate()?;
        let id = self
            .dependency_provider
            .database()
            .signup_id_gen()
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
    fn new(dependency_provider: Arc<D>) -> Self {
        Self {
            dependency_provider,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Policy;
    use crate::gateway::database::signup_process::{self, Record as SignupProcessRepoRecord};
    use crate::gateway::mock::MockDependencyProvider;
    use crate::usecase::tests::fixtures::*;
    use ca_domain::entity::auth_context::AuthContext;
    use ca_domain::entity::signup_process::SignupStateEnum;
    use rstest::rstest;

    #[rstest]
    #[tokio::test]
    async fn test_initialize_success(mut dependency_provider: MockDependencyProvider) {
        // Fixtures
        let id = Id::new(uuid::Uuid::new_v4());
        let record = SignupProcessRepoRecord {
            id,
            state: SignupStateEnum::Initialized {
                email: Email::new(TEST_EMAIL),
            },
            entered_at: chrono::Utc::now(),
        };
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        // Mock setup -- predicates and return values

        dependency_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(move || Ok(id));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record)
            .times(1)
            .returning(|_, _| Ok(()));
        // Usecase Initialization
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution is successful
        assert!(result.is_ok());
        // Assert return id equals the mock returned id.
        assert_eq!(result.unwrap().id, id);
    }

    #[rstest]
    #[tokio::test]
    async fn test_initialize_rejects_invalid_email(dependency_provider: MockDependencyProvider) {
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        let req = super::Request {
            email: "ttt".to_string(),
        };
        let result = usecase.exec(Authorized::for_test(req)).await;
        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("Validation error: email"));
        assert!(error.contains("Validation error: length"));
    }

    #[rstest]
    #[tokio::test]
    async fn test_initialize_fails_signup_id_gen(mut dependency_provider: MockDependencyProvider) {
        dependency_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(|| Err(NewIdError));
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let result = usecase.exec(Authorized::for_test(req)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::NewId);
    }

    #[rstest]
    #[tokio::test]
    async fn test_initialize_fails_save_latest_state(
        mut dependency_provider: MockDependencyProvider,
        signup_id: Id,
    ) {
        dependency_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(move || Ok(signup_id));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .returning(|_, _| Err(signup_process::SaveError::Connection));
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let result = usecase.exec(Authorized::for_test(req)).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::Repo,);
    }

    #[rstest]
    fn test_authorize_admin_zero(auth_context_admin: AuthContext) {
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let result =
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_admin),
            );
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(auth_context_user: AuthContext) {
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let result =
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_user),
            );
        assert!(result.is_ok());
    }
    #[rstest]
    fn test_authorize_none() {
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let auth_context = None;
        let result =
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                auth_context,
            );
        assert!(result.is_ok());
    }
}
