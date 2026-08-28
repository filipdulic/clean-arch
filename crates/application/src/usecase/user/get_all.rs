use std::sync::Arc;

use crate::auth::{AdminOnly, Authorized};
use crate::{
    gateway::{
        DatabaseProvider,
        database::{
            Database,
            user::{GetAllError, Repo},
        },
    },
    usecase::Usecase,
};
use ca_domain::entity::user::User;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request;

#[derive(Debug, Serialize)]
pub struct Response {
    pub users: Vec<User>,
}

/// Get all users usecase interactor
pub struct GetAll<D> {
    dependency_provider: Arc<D>,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("{}", GetAllError::Connection)]
    Repo,
}

impl From<GetAllError> for Error {
    fn from(e: GetAllError) -> Self {
        match e {
            GetAllError::Connection | GetAllError::InvalidData => Self::Repo,
        }
    }
}
#[async_trait::async_trait]
impl<D> Usecase<D> for GetAll<D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Auth = AdminOnly;

    async fn exec(&self, _req: Authorized<Self::Request>) -> Result<Self::Response, Self::Error> {
        log::debug!("Get all users");
        let users = self
            .dependency_provider
            .database()
            .user_repo()
            .get_all(None)
            .await?
            .into_iter()
            .map(User::from)
            .collect();
        Ok(Self::Response { users })
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
    async fn test_get_all_success(
        mut dependency_provider: MockDependencyProvider,
        user_records: Vec<UserRecord>,
    ) {
        // fixtures
        let req = Request;
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_all()
            .times(1)
            .returning(move |_| Ok(user_records.clone()));
        // Usecase Initialization
        let usecase = <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution success
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.users.len(), 2);
    }
    #[rstest]
    #[tokio::test]
    async fn test_get_all_success_return_empty(mut dependency_provider: MockDependencyProvider) {
        // fixtures
        let req = Request;
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_all()
            .times(1)
            .returning(move |_| Ok(vec![]));
        // Usecase Initialization
        let usecase = <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution success
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.users.is_empty());
    }
    #[rstest]
    #[tokio::test]
    async fn test_get_one_fail_get_all_connection(mut dependency_provider: MockDependencyProvider) {
        // fixtures
        let req = Request;
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_all()
            .times(1)
            .returning(move |_| Err(GetAllError::Connection));
        // Usecase Initialization
        let usecase = <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            Arc::new(dependency_provider),
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(Authorized::for_test(req)).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    fn test_authorize_admin_zero(auth_context_admin: AuthContext) {
        let req = Request;
        let result =
            <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_admin),
            );
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(auth_context_user: AuthContext) {
        let req = Request;
        let result =
            <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                Some(auth_context_user),
            );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_none() {
        let req = Request;
        let auth_context = None;
        let result =
            <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::Auth::check(
                req,
                auth_context,
            );
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
