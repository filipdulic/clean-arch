use std::sync::Arc;

use crate::{
    gateway::{
        database::{
            user::{GetError, Repo},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};
use ca_domain::entity::{
    auth_strategy::AuthStrategy,
    user::{Id, User},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub user: User,
}

/// Get all users usecase interactor
pub struct GetOne<D> {
    dependency_provider: Arc<D>,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("{}", GetError::NotFound)]
    NotFound,
    #[error("{}", GetError::Connection)]
    Repo,
}

impl From<GetError> for Error {
    fn from(e: GetError) -> Self {
        match e {
            GetError::Connection => Self::Repo,
            GetError::NotFound => Self::NotFound,
        }
    }
}
#[async_trait::async_trait]
impl<D> Usecase<D> for GetOne<D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Get user by ID");
        let user = self
            .dependency_provider
            .database()
            .user_repo()
            .get(None, req.id)
            .await?
            .into();
        Ok(Self::Response { user })
    }

    fn new(dependency_provider: Arc<D>) -> Self {
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
    use ca_domain::{
        entity::auth_context::{AuthContext, AuthError},
        value_object::Role,
    };
    use rstest::*;

    #[rstest]
    async fn test_get_one_success(
        mut dependency_provider: MockDependencyProvider,
        user_record: UserRecord,
    ) {
        // fixtures
        let req = Request {
            id: user_record.user.id(),
        };
        let user_id = user_record.user.id();
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Ok(user_record.clone()));
        // Usecase Initialization
        let usecase = <GetOne<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.user.id(), user_id);
        assert_eq!(result.user.username().to_string(), TEST_USERNAME);
        assert_eq!(result.user.email().to_string(), TEST_EMAIL);
        assert_eq!(result.user.role(), &Role::User);
        assert_eq!(result.user.password().to_string(), TEST_PASSWORD);
    }
    #[rstest]
    async fn test_get_one_fail_get_connection(
        mut dependency_provider: MockDependencyProvider,
        user_record: UserRecord,
    ) {
        // fixtures
        let req = Request {
            id: user_record.user.id(),
        };
        let user_id = user_record.user.id();
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Err(GetError::Connection));
        // Usecase Initialization
        let usecase = <GetOne<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    async fn test_get_one_fail_get_not_found(
        mut dependency_provider: MockDependencyProvider,
        user_record: UserRecord,
    ) {
        // fixtures
        let req = Request {
            id: user_record.user.id(),
        };
        let user_id = user_record.user.id();
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Err(GetError::NotFound));
        // Usecase Initialization
        let usecase = <GetOne<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NotFound);
    }
    #[rstest]
    fn test_authorize_admin_zero(user_record: UserRecord, auth_context_admin: AuthContext) {
        let req = Request {
            id: Id::new(user_record.user.id()),
        };
        let result = GetOne::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(user_record: UserRecord, auth_context_user: AuthContext) {
        let req = Request {
            id: user_record.user.id(),
        };
        let result = GetOne::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_user_owner(user_record: UserRecord, mut auth_context_user: AuthContext) {
        let req = Request {
            id: user_record.user.id(),
        };
        auth_context_user.user_id = user_record.user.id();
        let result = GetOne::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_ok());
    }
    #[rstest]
    fn test_authorize_none(user_record: UserRecord) {
        let req = Request {
            id: user_record.user.id(),
        };
        let auth_context = None;
        let result =
            GetOne::new(Arc::new(MockDependencyProvider::default())).authorize(&req, auth_context);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
