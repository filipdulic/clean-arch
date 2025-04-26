use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::signup_process::{EmailVerified, Failed, Id, SignupProcess};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Id,
}
pub struct ExtendCompletionTime<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
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
            GetError::IncorrectState => Self::IncorrectState(id),
            GetError::Connection => Self::Repo,
        }
    }
}
#[async_trait::async_trait]
impl<'d, D> Usecase<'d, D> for ExtendCompletionTime<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completion extended: {:?}", req);
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<Failed<EmailVerified>> =
            record.try_into().map_err(|err| (err, req.id))?;
        let process = process.recover();
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, process.into())
            .await?;
        Ok(Self::Response { id: req.id })
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
    async fn test_extend_completion_time_success(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        failed_verification_email_verified_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        let process: SignupProcess<Failed<EmailVerified>> =
            failed_verification_email_verified_record
                .clone()
                .try_into()
                .unwrap();
        // record to be passed to the save latest state method
        let record_to_save = process.recover().into();
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| Ok(failed_verification_email_verified_record.clone()));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(move |_, _| Ok(()));
        // Usecase Initialization
        let usecase = <ExtendCompletionTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(&dependency_provider);
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, signup_id);
    }
    #[rstest]
    async fn test_extend_completion_time_fail_get_latest_state_connection(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns a connection error
            .returning(move |_, _| Err(GetError::Connection));
        // Usecase Initialization
        let usecase = <ExtendCompletionTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(&dependency_provider);
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        // Assert error GetError::Connection is converted to Error::Repo
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    async fn test_extend_completion_time_fail_get_latest_state_not_found(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the error not found
            .returning(move |_, _| Err(GetError::NotFound));
        // Usecase Initialization
        let usecase = <ExtendCompletionTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(&dependency_provider);
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        // Assert error GetError::NotFound is converted to Error::NotFound
        // and correct id is passed
        assert_eq!(result.unwrap_err(), Error::NotFound(signup_id));
    }
    #[rstest]
    async fn test_extend_completion_time_fail_incorrect_state(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        initialized_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the incorrect state
            .returning(move |_, _| Ok(initialized_record.clone()));
        // Usecase Initialization
        let usecase = <ExtendCompletionTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(&dependency_provider);
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        // Assert error IncorrectState is returned
        assert_eq!(result.unwrap_err(), Error::IncorrectState(signup_id));
    }
    #[rstest]
    async fn test_extend_completion_time_fail_save_latest_state_connection(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        failed_verification_email_verified_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        let process: SignupProcess<Failed<EmailVerified>> =
            failed_verification_email_verified_record
                .clone()
                .try_into()
                .unwrap();
        let record_to_save = process.recover().into();
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| Ok(failed_verification_email_verified_record.clone()));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            // returns a connection error
            .returning(move |_, _| Err(SaveError::Connection));
        // Usecase Initialization
        let usecase = <ExtendCompletionTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(&dependency_provider);
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        // Assert error SaveError::Connection is converted to Error::Repo
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    fn test_authorize_admin_zero_success(signup_id: SignupId, auth_context_admin: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = ExtendCompletionTime::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_fail(signup_id: SignupId, auth_context_user: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = ExtendCompletionTime::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_none_fail(signup_id: SignupId) {
        let req = super::Request { id: signup_id };
        let result =
            ExtendCompletionTime::new(&MockDependencyProvider::default()).authorize(&req, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
