use std::sync::Arc;

use crate::auth::{AdminOnly, Authorized};
use crate::{
    gateway::{
        DatabaseProvider,
        database::{
            Database,
            user::{DeleteError, Repo},
        },
    },
    usecase::Usecase,
};

use ca_domain::entity::user::Id;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug, Serialize)]
pub struct Response;

/// Delete a user by ID.
pub struct Delete<D> {
    dependency_provider: Arc<D>,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("{}", DeleteError::NotFound)]
    NotFound,
    #[error("{}", DeleteError::Connection)]
    Repo,
}

impl From<DeleteError> for Error {
    fn from(e: DeleteError) -> Self {
        match e {
            DeleteError::NotFound => Self::NotFound,
            DeleteError::Connection => Self::Repo,
        }
    }
}
#[async_trait::async_trait]
impl<D> Usecase<D> for Delete<D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Auth = AdminOnly;

    async fn exec(&self, req: Authorized<Self::Request>) -> Result<Self::Response, Self::Error> {
        let req = req.into_request();
        log::debug!("Delete User by ID: {:?}", req);
        self.dependency_provider
            .database()
            .user_repo()
            .delete(None, req.id)
            .await?;
        Ok(Self::Response {})
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
    use ca_domain::entity::auth_context::{AuthContext, AuthError};
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn test_delete_success(mut dependency_provider: MockDependencyProvider, user_id: Id) {
        // fixtures
        let req = Request { id: user_id };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_delete()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Ok(()));
        // Usecase Initialization
        let usecase = <Delete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution success
        assert!(result.is_ok());
    }
    #[rstest]
    #[tokio::test]
    async fn test_delete_connection(mut dependency_provider: MockDependencyProvider, user_id: Id) {
        // fixtures
        let req = Request { id: user_id };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_delete()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Err(DeleteError::Connection));
        // Usecase Initialization
        let usecase = <Delete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    #[tokio::test]
    async fn test_delete_not_found(mut dependency_provider: MockDependencyProvider, user_id: Id) {
        // fixtures
        let req = Request { id: user_id };
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_delete()
            .withf(move |_, actual_id| actual_id == &user_id)
            .times(1)
            .returning(move |_, _| Err(DeleteError::NotFound));
        // Usecase Initialization
        let usecase = <Delete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NotFound);
    }

    #[rstest]
    fn test_authorize_admin_zero(user_record: UserRecord, auth_context_admin: AuthContext) {
        let req = Request {
            id: Id::new(user_record.user.id()),
        };
        let result =
            <Delete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_admin),
            );
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(user_record: UserRecord, auth_context_user: AuthContext) {
        let req = Request {
            id: user_record.user.id(),
        };
        let result =
            <Delete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_user),
            );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_none(user_record: UserRecord) {
        let req = Request {
            id: user_record.user.id(),
        };
        let auth_context = None;
        let result =
            <Delete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                auth_context,
            );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
