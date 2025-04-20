use crate::{
    gateway::{
        database::{
            identifier::{NewId, NewIdError},
            signup_process::{Repo, SaveError},
            Database,
        },
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
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    #[validate(email)]
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
        // validate email
        req.validate()?;
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
    use crate::gateway::database::signup_process::{self, Record as SignupProcessRepoRecord};
    use crate::gateway::mock::MockDependencyProvider;
    use crate::usecase::signup_process::fixtures::*;
    use ca_domain::entity::signup_process::SignupStateEnum;
    use rstest::rstest;

    #[rstest]
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
            .returning(move || Box::pin(async move { Ok(id) }));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record)
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(()) }));
        // Usecase Initialization
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution is successful
        assert!(result.is_ok());
        // Assert return id equals the mock returned id.
        assert_eq!(result.unwrap().id, id);
    }

    #[rstest]
    async fn test_initialize_fails_verify_email_min_lenght(
        dependency_provider: MockDependencyProvider,
    ) {
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        let req = super::Request {
            email: "ttt".to_string(),
        };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "email: Validation error: email [{\"value\": String(\"ttt\")}]"
        );
    }

    #[rstest]
    async fn test_initialize_fails_signup_id_gen(mut dependency_provider: MockDependencyProvider) {
        dependency_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(|| Box::pin(async { Err(NewIdError) }));
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::NewId);
    }

    #[rstest]
    async fn test_initialize_fails_save_latest_state(
        mut dependency_provider: MockDependencyProvider,
        signup_id: Id,
    ) {
        dependency_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(move || Box::pin(async move { Ok(signup_id) }));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .returning(|_, _| Box::pin(async { Err(signup_process::SaveError::Connection) }));
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::Repo,);
    }

    #[rstest]
    fn test_authorize_admin_zero(auth_context_admin: AuthContext) {
        let req = super::Request {
            email: TEST_EMAIL.to_string(),
        };
        let result =
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::authorize(
                &req,
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
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::authorize(
                &req,
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
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::authorize(
                &req,
                auth_context,
            );
        assert!(result.is_ok());
    }
}
