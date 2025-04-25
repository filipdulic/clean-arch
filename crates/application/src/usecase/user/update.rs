use crate::{
    gateway::{
        database::{
            user::{GetError, Repo, SaveError},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};
use ca_domain::{
    entity::{
        auth_strategy::AuthStrategy,
        user::{Email, Id, UserName},
    },
    value_object::Password,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    pub id: Id,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, max = 30))]
    pub username: String,
    #[validate(length(min = 5, max = 60))]
    pub password: String,
}

pub type Response = ();

pub struct Update<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("User {0} not found")]
    NotFound(Id),
    #[error(transparent)]
    Invalidity(#[from] validator::ValidationErrors),
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
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for Update<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Update User: {:?}", req);
        req.validate()?;
        let mut record = self
            .dependency_provider
            .database()
            .user_repo()
            .get(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        record.user.update(
            Email::new(&req.email),
            UserName::new(&req.username),
            Password::new(&req.password),
        );
        self.dependency_provider
            .database()
            .user_repo()
            .save(None, record)
            .await?;
        Ok(())
    }

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn auth_strategy(&self) -> AuthStrategy {
        AuthStrategy::AdminAndOwnerOnly
    }
    fn extract_owner(&self, req: &Self::Request) -> Option<Id> {
        Some(req.id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        gateway::{database::user::Record as UserRecord, mock::MockDependencyProvider},
        usecase::tests::fixtures::*,
    };
    use ca_domain::entity::auth_context::{AuthContext, AuthError};
    use rstest::*;

    #[rstest]
    async fn test_update_success(
        mut dependency_provider: MockDependencyProvider,
        user_id: Id,
        user_record: UserRecord,
    ) {
        // fixtures
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let mut updated_user_record = user_record.clone();
        updated_user_record.user.update(
            Email::new(&req.email),
            UserName::new(&req.username),
            Password::new(&req.password),
        );
        let expected_user_record = user_record.clone();
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Ok(user_record.clone()));
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_save()
            .withf(move |_, actual_record| actual_record == &expected_user_record)
            .times(1)
            .returning(move |_, _| Ok(()));
        // Usecase Initialization
        let usecase = <Update<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
    }
    #[rstest]
    async fn test_update_fail_req_validation(
        dependency_provider: MockDependencyProvider,
        user_id: Id,
    ) {
        // fixtures
        let req = Request {
            id: user_id,
            email: "".to_string(),
            username: "".to_string(),
            password: "".to_string(),
        };
        // Usecase Initialization
        let usecase = <Update<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("email: Validation error: email"));
        assert!(err
            .to_string()
            .contains("password: Validation error: length"));
        assert!(err
            .to_string()
            .contains("username: Validation error: length"));
    }
    #[rstest]
    async fn test_update_fail_get_connection(
        mut dependency_provider: MockDependencyProvider,
        user_id: Id,
    ) {
        // fixtures
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Err(GetError::Connection));
        // Usecase Initialization
        let usecase = <Update<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    async fn test_update_fail_get_not_found(
        mut dependency_provider: MockDependencyProvider,
        user_id: Id,
    ) {
        // fixtures
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Err(GetError::NotFound));
        // Usecase Initialization
        let usecase = <Update<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NotFound(user_id));
    }
    #[rstest]
    async fn test_update_fail_save_connection(
        mut dependency_provider: MockDependencyProvider,
        user_id: Id,
        user_record: UserRecord,
    ) {
        // fixtures
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let mut updated_user_record = user_record.clone();
        updated_user_record.user.update(
            Email::new(&req.email),
            UserName::new(&req.username),
            Password::new(&req.password),
        );
        let expected_user_record = user_record.clone();
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Ok(user_record.clone()));
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_save()
            .withf(move |_, actual_record| actual_record == &expected_user_record)
            .times(1)
            .returning(move |_, _| Err(SaveError::Connection));
        // Usecase Initialization
        let usecase = <Update<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    fn test_authorize_admin_zero(user_id: Id, auth_context_admin: AuthContext) {
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let result = Update::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(user_id: Id, auth_context_user: AuthContext) {
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let result = Update::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_user_owner(user_id: Id, mut auth_context_user: AuthContext) {
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        auth_context_user.user_id = user_id;
        let result = Update::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_ok());
    }
    #[rstest]
    fn test_authorize_none(user_id: Id) {
        let req = Request {
            id: user_id,
            email: TEST_EMAIL.to_string(),
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let auth_context = None;
        let result = Update::new(&MockDependencyProvider::default()).authorize(&req, auth_context);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
