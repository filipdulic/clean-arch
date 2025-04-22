use crate::{
    gateway::{
        database::{
            user::{GetAllError, Repo},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};
use ca_domain::entity::{auth_strategy::AuthStrategy, user::User};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request;

#[derive(Debug, Serialize)]
pub struct Response {
    pub users: Vec<User>,
}

/// Get all users usecase interactor
pub struct GetAll<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("{}", GetAllError::Connection)]
    Repo,
}

impl From<GetAllError> for Error {
    fn from(e: GetAllError) -> Self {
        match e {
            GetAllError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for GetAll<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, _req: Self::Request) -> Result<Self::Response, Self::Error> {
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

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn auth_strategy(&self) -> AuthStrategy {
        AuthStrategy::AdminOnly
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
            .returning(move |_| {
                Box::pin({
                    let records = user_records.clone();
                    async move { Ok(records) }
                })
            });
        // Usecase Initialization
        let usecase = <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.users.len(), 2);
    }
    #[rstest]
    async fn test_get_all_success_return_empty(mut dependency_provider: MockDependencyProvider) {
        // fixtures
        let req = Request;
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_all()
            .times(1)
            .returning(move |_| Box::pin(async move { Ok(vec![]) }));
        // Usecase Initialization
        let usecase = <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.users.is_empty());
    }
    #[rstest]
    async fn test_get_one_fail_get_all_connection(mut dependency_provider: MockDependencyProvider) {
        // fixtures
        let req = Request;
        // mock setup
        dependency_provider
            .db
            .user_repo
            .expect_get_all()
            .times(1)
            .returning(move |_| Box::pin(async move { Err(GetAllError::Connection) }));
        // Usecase Initialization
        let usecase = <GetAll<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    fn test_authorize_admin_zero(auth_context_admin: AuthContext) {
        let req = Request;
        let result = GetAll::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(auth_context_user: AuthContext) {
        let req = Request;
        let result = GetAll::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_none() {
        let req = Request;
        let auth_context = None;
        let result = GetAll::new(&MockDependencyProvider::default()).authorize(&req, auth_context);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
