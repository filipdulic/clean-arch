use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Record, Repo},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::signup_process::Id;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub state_chain: Vec<Record>,
}

pub struct GetStateChain<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
    #[error("{}", GetError::Connection)]
    Repo,
}

impl From<(GetError, Id)> for Error {
    fn from((err, id): (GetError, Id)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(id),
            GetError::IncorrectState => Self::IncorrectState(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for GetStateChain<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("Get signup process state chain");
        let state_chain = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_state_chain(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        Ok(Self::Response { state_chain })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        gateway::{
            database::signup_process::Record as SignupProcessRepoRecord,
            mock::MockDependencyProvider,
        },
        usecase::tests::fixtures::*,
    };
    use ca_domain::entity::{
        auth_context::{AuthContext, AuthError},
        signup_process::Id as SignupId,
    };
    use rstest::*;

    #[rstest]
    async fn test_get_state_chain_success(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        state_chain_record_vector: Vec<SignupProcessRepoRecord>,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        let expected = state_chain_record_vector.clone();
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_state_chain()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| {
                Box::pin({
                    let record = state_chain_record_vector.clone();
                    async move { Ok(record) }
                })
            });
        // Usecase Initialization
        let usecase =
            <GetStateChain<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
                &dependency_provider,
            );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let state_chain = result.unwrap().state_chain;
        assert_eq!(expected, state_chain);
    }
    #[rstest]
    async fn test_get_state_chain_fail_connection(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_state_chain()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| Box::pin(async move { Err(GetError::Connection) }));
        // Usecase Initialization
        let usecase =
            <GetStateChain<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
                &dependency_provider,
            );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo,);
    }
    #[rstest]
    async fn test_get_state_chain_fail_not_found(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_state_chain()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| Box::pin(async move { Err(GetError::NotFound) }));
        // Usecase Initialization
        let usecase =
            <GetStateChain<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
                &dependency_provider,
            );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NotFound(signup_id),);
    }
    #[rstest]
    fn test_authorize_admin_zero_success(signup_id: SignupId, auth_context_admin: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = GetStateChain::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }
    #[rstest]
    fn test_authorize_user_fail(signup_id: SignupId, auth_context_user: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = GetStateChain::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_none_fail(signup_id: SignupId) {
        let req = super::Request { id: signup_id };
        let result = GetStateChain::new(&MockDependencyProvider::default()).authorize(&req, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
