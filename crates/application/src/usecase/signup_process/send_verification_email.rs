use std::sync::Arc;

use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            token::{GenError as TokenRepoError, Repo as TokenRepo},
            Database,
        },
        service::email::{EmailAddress, EmailServiceError, EmailVerificationService},
        DatabaseProvider, EmailVerificationServiceProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::signup_process::{
    Error as SignupProcessError, Id, Initialized, SignupProcess,
};
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
pub struct SendVerificationEmail<D> {
    dependency_provider: Arc<D>,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
    #[error("SignupProcess Repo error")]
    Repo,
    #[error("Token Repo error: {0}")]
    TokenRepoError(#[from] TokenRepoError),
    #[error("Email Service error: {0}")]
    EmailServiceError(#[from] EmailServiceError),
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

impl From<SaveError> for Error {
    fn from(_: SaveError) -> Self {
        Self::Repo
    }
}
#[async_trait::async_trait]
impl<D> Usecase<D> for SendVerificationEmail<D>
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess SendVerificationEmail ID: {:?}", req);
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<Initialized> = record.try_into().map_err(|err| (err, req.id))?;
        let token = match self
            .dependency_provider
            .database()
            .token_repo()
            .gen(None, process.state().email.as_ref())
            .await
        {
            Ok(record) => record.token,
            Err(err) => {
                log::error!("Token Repo error: {:?}", err);
                let process = process.fail(SignupProcessError::TokenGenrationFailed);
                self.dependency_provider
                    .database()
                    .signup_process_repo()
                    .save_latest_state(None, process.into())
                    .await?;
                return Err(err.into());
            }
        };
        if let Err(err) = self
            .dependency_provider
            .email_verification_service()
            .send_verification_email(
                EmailAddress::new(process.state().email.as_ref()),
                token.as_str(),
            )
            .await
        {
            log::error!("Email Service error: {:?}", err);
            let process = process.fail(SignupProcessError::VerificationEmailSendError);
            self.dependency_provider
                .database()
                .signup_process_repo()
                .save_latest_state(None, process.into())
                .await?;
            return Err(err.into());
        }
        let process = process.send_verification_email();
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, process.into())
            .await?;
        Ok(Response { id: req.id })
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
    use crate::gateway::database::signup_process::{self, Record as SignupProcessRepoRecord};
    use crate::gateway::database::token::{GenError as TokenRepoError, Record as TokenRepoRecord};
    use crate::gateway::mock::MockDependencyProvider;
    use crate::usecase::tests::fixtures::*;
    use ca_domain::entity::auth_context::{AuthContext, AuthError};
    use ca_domain::entity::signup_process::{
        Error as SignupProcessError, Id as SignupId, SignupStateEnum,
    };
    use rstest::*;

    #[rstest]
    async fn test_send_verification_email_success(
        mut dependency_provider: MockDependencyProvider,
        initialized_record: SignupProcessRepoRecord,
        token_repo_record: TokenRepoRecord,
    ) {
        // fixtures
        let process = SignupProcess::<Initialized>::try_from(initialized_record.clone()).unwrap();
        let process = process.send_verification_email();
        let token = token_repo_record.token.clone();
        let record_to_save: SignupProcessRepoRecord = process.clone().into();
        let id = initialized_record.id;
        let req = super::Request {
            id: initialized_record.id,
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &initialized_record.id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| Ok(initialized_record.clone()));
        dependency_provider
            .db
            .token_repo
            .expect_gen()
            // makes sure the correct email is used
            .withf(move |_, actual_email| actual_email == TEST_EMAIL)
            .times(1)
            // returns test_token to simulate token generation success
            .returning(move |_, _| Ok(token_repo_record.clone()));
        dependency_provider
            .email_verification_service
            .expect_send_verification_email()
            // makes sure the correct email and token are used
            .withf(move |actual_email, actual_token| {
                actual_email.as_str() == TEST_EMAIL && actual_token == token.as_str()
            })
            .times(1)
            // returns ok to simulate email send success
            .returning(|_, _| Ok(()));
        dependency_provider
            .db
            .signup_process_repo
            // all steps successful, so we save the process in the new state
            .expect_save_latest_state()
            // makes sure the correct process in failed state is saved
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(|_, _| Ok(()));
        // Usecase Initialization
        let usecase = <SendVerificationEmail<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, id);
    }
    #[rstest]
    async fn test_send_verification_email_fails_get_latest_state_connection(
        signup_id: SignupId,
        mut dependency_provider: MockDependencyProvider,
    ) {
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            .returning(|_, _| Err(signup_process::GetError::Connection));
        let usecase = <SendVerificationEmail<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        let req = super::Request { id: signup_id };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::Repo,);
    }
    #[rstest]
    async fn test_send_verification_email_fails_get_latest_state_not_found(
        signup_id: SignupId,
        mut dependency_provider: MockDependencyProvider,
    ) {
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            .returning(|_, _| Err(signup_process::GetError::NotFound));
        let usecase = <SendVerificationEmail<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        let req = super::Request { id: signup_id };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::NotFound(signup_id),);
    }
    #[rstest]
    async fn test_send_verification_email_fails_incorrect_state(
        signup_id: SignupId,
        mut dependency_provider: MockDependencyProvider,
    ) {
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            .returning(move |_, _| {
                Ok(SignupProcessRepoRecord {
                    id: signup_id,
                    state: SignupStateEnum::ForDeletion, // should be Initialized
                    entered_at: chrono::Utc::now(),
                })
            });
        let usecase = <SendVerificationEmail<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        let req = super::Request { id: signup_id };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), super::Error::IncorrectState(signup_id),);
    }
    #[rstest]
    async fn test_send_verification_email_fails_token_gen(
        mut dependency_provider: MockDependencyProvider,
        initialized_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let process = SignupProcess::<Initialized>::try_from(initialized_record.clone())
            .unwrap()
            .fail(SignupProcessError::TokenGenrationFailed);
        let record_to_save: SignupProcessRepoRecord = process.clone().into();
        let req = super::Request {
            id: initialized_record.id,
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &initialized_record.id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| Ok(initialized_record.clone()));
        dependency_provider
            .db
            .token_repo
            .expect_gen()
            // makes sure the correct email is used
            .withf(move |_, actual_email| actual_email == TEST_EMAIL)
            .times(1)
            // returns an error to simulate token generation failure
            .returning(|_, _| Err(TokenRepoError::Connection));
        dependency_provider
            .db
            .signup_process_repo
            // token generation failed, so we save the process with the error
            .expect_save_latest_state()
            // makes sure the correct process in failed state is saved
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(|_, _| Ok(()));
        // Usecase Initialization
        let usecase = <SendVerificationEmail<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution failed with TokenRepoError
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            super::Error::TokenRepoError(TokenRepoError::Connection),
        );
    }
    #[rstest]
    async fn test_send_verification_email_fails_email_send(
        mut dependency_provider: MockDependencyProvider,
        initialized_record: SignupProcessRepoRecord,
        token_repo_record: TokenRepoRecord,
    ) {
        // fixtures
        let process = SignupProcess::<Initialized>::try_from(initialized_record.clone())
            .unwrap()
            .fail(SignupProcessError::VerificationEmailSendError);
        let token = token_repo_record.token.clone();
        let record_to_save: SignupProcessRepoRecord = process.clone().into();
        let req = super::Request {
            id: initialized_record.id,
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &initialized_record.id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| Ok(initialized_record.clone()));
        dependency_provider
            .db
            .token_repo
            .expect_gen()
            // makes sure the correct email is used
            .withf(move |_, actual_email| actual_email == TEST_EMAIL)
            .times(1)
            // returns test_token to simulate token generation success
            .returning(move |_, _| Ok(token_repo_record.clone()));
        dependency_provider
            .email_verification_service
            .expect_send_verification_email()
            // makes sure the correct email and token are used
            .withf(move |actual_email, actual_token| {
                actual_email.as_str() == TEST_EMAIL && actual_token == token.as_str()
            })
            .times(1)
            // returns an error to simulate email send failure
            .returning(|_, _| Err(EmailServiceError::SendEmailFailed));
        dependency_provider
            .db
            .signup_process_repo
            // token generation failed, so we save the process with the error
            .expect_save_latest_state()
            // makes sure the correct process in failed state is saved
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(|_, _| Ok(()));
        // Usecase Initialization
        let usecase = <SendVerificationEmail<MockDependencyProvider> as Usecase<
            MockDependencyProvider,
        >>::new(Arc::new(dependency_provider));
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution failed with TokenRepoError
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            super::Error::EmailServiceError(EmailServiceError::SendEmailFailed),
        );
    }
    #[rstest]
    fn test_authorize_admin_zero(signup_id: SignupId, auth_context_admin: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = SendVerificationEmail::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user(signup_id: SignupId, auth_context_user: AuthContext) {
        let req = super::Request { id: signup_id };
        let result = SendVerificationEmail::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
    #[rstest]
    fn test_authorize_none(signup_id: SignupId) {
        let req = super::Request { id: signup_id };
        let result = SendVerificationEmail::new(Arc::new(MockDependencyProvider::default()))
            .authorize(&req, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), AuthError::Unauthorized);
    }
}
