use std::sync::Arc;

use crate::auth::{Authorized, Public};
use crate::{
    gateway::{
        AuthPackerProvider, DatabaseProvider,
        database::{
            Database,
            user::{GetError, Repo, SaveError},
        },
        service::auth::{AuthPacker, AuthServiceError},
    },
    usecase::Usecase,
};
use ca_domain::entity::{
    auth_context::AuthContext,
    user::{Id, Password, UserName},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub user_id: Id,
    pub token: String,
}

pub struct Login<D> {
    dependency_provider: Arc<D>,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("User with username {0} not found")]
    NotFound(UserName),
    #[error("User password or username is invalid")]
    InvalidLogin,
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error(transparent)]
    Auth(#[from] AuthServiceError),
}

impl From<SaveError> for Error {
    fn from(err: SaveError) -> Self {
        match err {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<(GetError, UserName)> for Error {
    fn from((err, user_name): (GetError, UserName)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(user_name),
            GetError::Connection | GetError::InvalidData => Self::Repo,
        }
    }
}
#[async_trait::async_trait]
impl<D> Usecase<D> for Login<D>
where
    D: DatabaseProvider + AuthPackerProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Auth = Public;

    async fn exec(&self, req: Authorized<Self::Request>) -> Result<Self::Response, Self::Error> {
        let req = req.into_request();
        log::debug!("Login User: {:?}", req.username);
        let user_name = UserName::new(&req.username);
        let password = Password::new(&req.password);
        let record = self
            .dependency_provider
            .database()
            .user_repo()
            .get_by_username(None, user_name.clone())
            .await
            .map_err(|err| (err, user_name))?;
        // check password
        if password.ne(record.user.password()) {
            return Err(Error::InvalidLogin);
        }
        let auth_context = AuthContext::new(record.user.id(), record.user.role().clone());
        let token = self
            .dependency_provider
            .auth_packer()
            .pack_auth(auth_context)
            .await?;
        Ok(Response {
            user_id: record.user.id(),
            token,
        })
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
    use crate::{
        gateway::{database::user::Record as UserRecord, mock::MockDependencyProvider},
        usecase::tests::fixtures::*,
    };
    use ca_domain::entity::auth_context::AuthContext;
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn test_login_success(
        mut dependency_provider: MockDependencyProvider,
        user_record: UserRecord,
    ) {
        // fixtures
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let user_id = user_record.user.id();
        let auth_context = AuthContext::new(user_id, user_record.user.role().clone());
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_by_username()
            .withf(move |_, actual_username| actual_username == &UserName::new(TEST_USERNAME))
            .times(1)
            .returning(move |_, _| Ok(user_record.clone()));
        dependency_provider
            .auth_packer
            .expect_pack_auth()
            .withf(move |actual_auth_context| actual_auth_context == &auth_context)
            .times(1)
            .returning(move |_| Ok(TEST_TOKEN.to_string()));
        // Usecase Initialization
        let usecase = <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution success
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.user_id, user_id);
        assert_eq!(result.token, TEST_TOKEN);
    }

    #[rstest]
    #[tokio::test]
    async fn test_login_fails_when_token_packing_fails(
        mut dependency_provider: MockDependencyProvider,
        user_record: UserRecord,
    ) {
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        dependency_provider
            .db
            .user_repo
            .expect_get_by_username()
            .returning(move |_, _| Ok(user_record.clone()));
        dependency_provider
            .auth_packer
            .expect_pack_auth()
            .returning(|_| Err(AuthServiceError::Pack));
        let usecase = <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );

        assert_eq!(
            usecase.exec(Authorized::for_test(req)).await.unwrap_err(),
            Error::Auth(AuthServiceError::Pack)
        );
    }
    #[rstest]
    #[tokio::test]
    async fn test_login_fail_get_by_username_connection(
        mut dependency_provider: MockDependencyProvider,
    ) {
        // fixtures
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_by_username()
            .withf(move |_, actual_username| actual_username == &UserName::new(TEST_USERNAME))
            .times(1)
            .returning(move |_, _| Err(GetError::Connection));
        // Usecase Initialization
        let usecase = <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    #[tokio::test]
    async fn test_login_fail_get_by_username_not_found(
        mut dependency_provider: MockDependencyProvider,
    ) {
        // fixtures
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_by_username()
            .withf(move |_, actual_username| actual_username == &UserName::new(TEST_USERNAME))
            .times(1)
            .returning(move |_, _| Err(GetError::NotFound));
        // Usecase Initialization
        let usecase = <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::NotFound(UserName::new(TEST_USERNAME))
        );
    }
    #[rstest]
    #[tokio::test]
    async fn test_login_fail_get_by_username_invalid_password(
        mut dependency_provider: MockDependencyProvider,
        user_record: UserRecord,
    ) {
        // fixtures
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: "fail password".to_string(),
        };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_by_username()
            .withf(move |_, actual_username| actual_username == &UserName::new(TEST_USERNAME))
            .times(1)
            .returning(move |_, _| Ok(user_record.clone()));
        // Usecase Initialization
        let usecase = <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::InvalidLogin);
    }
    #[rstest]
    fn test_authorize_admin_zero(auth_context_admin: AuthContext) {
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let result =
            <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_admin),
            );
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(auth_context_user: AuthContext) {
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let result =
            <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_user),
            );
        assert!(result.is_ok());
    }
    #[rstest]
    fn test_authorize_none() {
        let req = Request {
            username: TEST_USERNAME.to_string(),
            password: TEST_PASSWORD.to_string(),
        };
        let auth_context = None;
        let result =
            <Login<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                auth_context,
            );
        assert!(result.is_ok());
    }
}
