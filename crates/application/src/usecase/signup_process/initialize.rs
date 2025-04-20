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
    use crate::gateway::database::mock::MockDependencyProvider;
    use crate::gateway::database::signup_process::{self, Record as SignupProcessRepoRecord};
    use ca_domain::entity::signup_process::SignupStateEnum;
    use ca_domain::entity::user;
    use ca_domain::value_object::Role;

    #[tokio::test]
    async fn test_initialize_success() {
        // Fixtures
        let id = Id::new(uuid::Uuid::new_v4());
        let email = "email@test.com".to_string();
        let record = SignupProcessRepoRecord {
            id,
            state: SignupStateEnum::Initialized {
                email: Email::new(email.clone()),
            },
            entered_at: chrono::Utc::now(),
        };
        let req = super::Request {
            email: email.clone(),
        };
        // Mock setup -- predicates and return values
        let mut db_provider = MockDependencyProvider::default();
        db_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(move || Box::pin(async move { Ok(id) }));
        db_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |transaction, actual_record| {
                // Check if the transaction is None
                // and the actual_record matches the expected record
                transaction.is_none() && actual_record == &record
            })
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(()) }));
        // Usecase Initialization
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &db_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution is successful
        assert!(result.is_ok());
        // Assert return id equals the mock returned id.
        assert_eq!(result.unwrap().id, id);
    }

    #[tokio::test]
    async fn test_initialize_fails_verify_email_min_lenght() {
        let db_provider = MockDependencyProvider::default();
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &db_provider,
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

    #[tokio::test]
    async fn test_initialize_fails_signup_id_gen() {
        let mut db_provider = MockDependencyProvider::default();
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

    #[tokio::test]
    async fn test_initialize_fails_save_latest_state() {
        let mut db_provider = MockDependencyProvider::default();
        let id = Id::new(uuid::Uuid::new_v4());
        db_provider
            .db
            .signup_id_gen
            .expect_new_id()
            .returning(move || Box::pin(async move { Ok(id) }));
        db_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .returning(|_, _| Box::pin(async { Err(signup_process::SaveError::Connection) }));
        let usecase = <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &db_provider,
        );
        let req = super::Request {
            email: "email@test.com".to_string(),
        };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            signup_process::SaveError::Connection.into(),
        );
    }

    #[test]
    fn test_authorize_admin_zero() {
        let req = super::Request {
            email: "email@test.com".to_string(),
        };
        let auth_context = Some(AuthContext {
            user_id: user::Id::new(uuid::Uuid::nil()),
            role: Role::Admin,
        });
        let result =
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::authorize(
                &req,
                auth_context,
            );
        assert!(result.is_ok());
    }

    #[test]
    fn test_authorize_user_zero() {
        let req = super::Request {
            email: "email@test.com".to_string(),
        };
        let auth_context = Some(AuthContext {
            user_id: user::Id::new(uuid::Uuid::nil()),
            role: Role::User,
        });
        let result =
            <Initialize<MockDependencyProvider> as Usecase<MockDependencyProvider>>::authorize(
                &req,
                auth_context,
            );
        assert!(result.is_ok());
    }
    #[test]
    fn test_authorize_none() {
        let req = super::Request {
            email: "email@test.com".to_string(),
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
