use std::sync::Arc;

use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            token::{ExtendError, Repo as TokenRepo},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::signup_process::{Failed, Id, SignupProcess, VerificationEmailSent};

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
pub struct ExtendVerificationTime<D> {
    dependency_provider: Arc<D>,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("Token Extension Error {0}")]
    TokenRepoError(#[from] ExtendError),
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
            GetError::IncorrectState => Self::IncorrectState(id),
            GetError::Connection => Self::Repo,
            GetError::NotFound => Self::NotFound(id),
        }
    }
}
#[async_trait::async_trait]
impl<D> Usecase<D> for ExtendVerificationTime<D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Verification extended: {:?}", req);
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        // check if the process is in the right state
        let process: SignupProcess<Failed<VerificationEmailSent>> =
            record.try_into().map_err(|err| (err, req.id))?;
        // update token
        let process = process.recover();
        self.dependency_provider
            .database()
            .token_repo()
            .extend(None, process.state().email.as_ref())
            .await?;
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, process.into())
            .await?;
        Ok(Self::Response { id: req.id })
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
    async fn test_extend_verification_time_success(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        failed_verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        let process: SignupProcess<Failed<VerificationEmailSent>> =
            failed_verification_email_sent_record
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
            .returning(move |_, _| Ok(failed_verification_email_sent_record.clone()));
        dependency_provider
            .db
            .token_repo
            .expect_extend()
            .withf(move |_, actual_email| actual_email == TEST_EMAIL)
            .times(1)
            .returning(move |_, _| Ok(()));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(move |_, _| Ok(()));
        // Usecase Initialization
        let usecase = <ExtendVerificationTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, signup_id);
    }
    #[rstest]
    async fn test_extend_verification_time_fail_get_latest_state_connection(
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
            // returns the connection error
            .returning(move |_, _| Err(GetError::Connection));
        // Usecase Initialization
        let usecase = <ExtendVerificationTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution has failed
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    async fn test_extend_verification_time_fail_get_latest_state_not_found(
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
            // returns the not found error
            .returning(move |_, _| Err(GetError::NotFound));
        // Usecase Initialization
        let usecase = <ExtendVerificationTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution has failed
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NotFound(signup_id));
    }
    #[rstest]
    async fn test_extend_verification_time_fails_incorrect_state(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        verification_email_sent_record: SignupProcessRepoRecord,
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
            .returning(move |_, _| Ok(verification_email_sent_record.clone()));
        // Usecase Initialization
        let usecase = <ExtendVerificationTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::IncorrectState(signup_id));
    }
    #[rstest]
    async fn test_extend_verification_time_fail_token_repo_extend(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        failed_verification_email_sent_record: SignupProcessRepoRecord,
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
            // returns the record with the correct state
            .returning(move |_, _| Ok(failed_verification_email_sent_record.clone()));
        dependency_provider
            .db
            .token_repo
            .expect_extend()
            .withf(move |_, actual_email| actual_email == TEST_EMAIL)
            .times(1)
            .returning(move |_, _| Err(ExtendError::Connection));
        // Usecase Initialization
        let usecase = <ExtendVerificationTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::TokenRepoError(ExtendError::Connection)
        );
    }
    #[rstest]
    async fn test_extend_verification_time_fail_save_latest_state_connection(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        failed_verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request { id: signup_id };
        let process: SignupProcess<Failed<VerificationEmailSent>> =
            failed_verification_email_sent_record
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
            .returning(move |_, _| Ok(failed_verification_email_sent_record.clone()));
        dependency_provider
            .db
            .token_repo
            .expect_extend()
            .withf(move |_, actual_email| actual_email == TEST_EMAIL)
            .times(1)
            .returning(move |_, _| Ok(()));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(move |_, _| Err(SaveError::Connection));
        // Usecase Initialization
        let usecase = <ExtendVerificationTime<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    fn test_authorize_admin_zero_success(signup_id: SignupId, auth_context_admin: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = ExtendVerificationTime::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_fail(signup_id: SignupId, auth_context_user: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = ExtendVerificationTime::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_none_fail(signup_id: SignupId) {
        let req = super::Request { id: signup_id };
        let result = ExtendVerificationTime::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
