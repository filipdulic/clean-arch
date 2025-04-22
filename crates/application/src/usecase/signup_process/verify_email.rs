use crate::{
    gateway::{
        database::{
            signup_process::{GetError, Repo, SaveError},
            token::{Repo as TokenRepo, VerifyError as TokenRepoError},
            Database,
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::{
    auth_strategy::AuthStrategy,
    signup_process::{Id, SignupProcess, VerificationEmailSent},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    pub id: Id,
    #[validate(length(min = 1, max = 255))]
    pub token: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub id: Id,
}
pub struct VerifyEmail<'d, D> {
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
    #[error("Token Repo error: {0}")]
    TokenRepoError(#[from] TokenRepoError),
    #[error(transparent)]
    TokenInvalidity(#[from] validator::ValidationErrors),
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

impl<'d, D> Usecase<'d, D> for VerifyEmail<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    /// Create a new user with the given name.
    async fn exec(&self, req: Request) -> Result<Response, Error> {
        log::debug!("SignupProcess Email Verification: {:?}", req);
        // Validate the request
        req.validate()?;
        // Begin transaction
        let mut transaction = self
            .dependency_provider
            .database()
            .begin_transaction()
            .await;
        // Load record
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(Some(&mut transaction), req.id)
            .await
            .map_err(|err| (err, req.id))?;
        let process: SignupProcess<VerificationEmailSent> =
            record.try_into().map_err(|err| (err, req.id))?;
        // Verify the token
        if let Err(err) = self
            .dependency_provider
            .database()
            .token_repo()
            .verify(
                Some(&mut transaction),
                process.state().email.as_ref(),
                &req.token,
            )
            .await
        {
            log::error!("Token Repo error: {:?}", err);
            if let TokenRepoError::TokenExpired = err {
                let process =
                    process.fail(ca_domain::entity::signup_process::Error::VerificationTimedOut);
                self.dependency_provider
                    .database()
                    .signup_process_repo()
                    .save_latest_state(Some(&mut transaction), process.into())
                    .await?;
            }
            self.dependency_provider
                .database()
                .commit_transaction(transaction)
                .await
                .map_err(|_| SaveError::Connection)?;
            return Err(err.into());
        };
        // Update the process state
        let process = process.verify_email();
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(Some(&mut transaction), process.into())
            .await?;
        self.dependency_provider
            .database()
            .commit_transaction(transaction)
            .await
            .map_err(|_| SaveError::Connection)?;
        Ok(Self::Response { id: req.id })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn auth_strategy(&self) -> AuthStrategy {
        AuthStrategy::Public
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{
        gateway::{
            database::signup_process::Record as SignupProcessRepoRecord,
            database::token::VerifyError, mock::MockDependencyProvider,
        },
        usecase::tests::fixtures::*,
    };
    use ca_domain::entity::{
        auth_context::AuthContext,
        signup_process::{Error as SignupError, Id as SignupId},
    };

    use super::*;

    #[rstest]
    async fn test_verify_email_success(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        let process: SignupProcess<VerificationEmailSent> =
            verification_email_sent_record.clone().try_into().unwrap();
        // record to be passed to the save latest state method
        let record_to_save = process.verify_email().into();
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| {
                Box::pin({
                    let record = verification_email_sent_record.clone();
                    async move { Ok(record) }
                })
            });
        dependency_provider
            .db
            .token_repo
            .expect_verify()
            // makes sure the correct token is used
            .withf(move |_, actual_email, actual_token| {
                actual_token == TEST_TOKEN && actual_email == TEST_EMAIL
            })
            .times(1)
            // returns Ok
            .returning(move |_, _, _| Box::pin(async move { Ok(()) }));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(()) }));
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.id, signup_id);
    }
    #[rstest]
    async fn test_verify_email_fails_verify_token_min_lenght(
        dependency_provider: MockDependencyProvider,
    ) {
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        let id = Id::from(uuid::Uuid::new_v4());
        let req = super::Request {
            id,
            token: "".to_string(),
        };
        let result = usecase.exec(req).await;
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("token: Validation error: length"));
    }
    #[rstest]
    async fn test_verify_email_fail_get_latest_state_connection(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns connection error
            .returning(move |_, _| Box::pin(async move { Err(GetError::Connection) }));
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    async fn test_verify_email_fail_get_latest_state_not_found(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns connection error
            .returning(move |_, _| Box::pin(async move { Err(GetError::NotFound) }));
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::NotFound(signup_id));
    }
    #[rstest]
    async fn test_verify_email_fail_get_latest_state_incorrect_state(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        initialized_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns record in incorrect state
            .returning(move |_, _| {
                Box::pin({
                    let record = initialized_record.clone();
                    async move { Ok(record) }
                })
            });
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::IncorrectState(signup_id));
    }
    #[rstest]
    async fn test_verify_fail_token_verification_connection(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| {
                Box::pin({
                    let record = verification_email_sent_record.clone();
                    async move { Ok(record) }
                })
            });
        dependency_provider
            .db
            .token_repo
            .expect_verify()
            // makes sure the correct token is used
            .withf(move |_, actual_email, actual_token| {
                actual_token == TEST_TOKEN && actual_email == TEST_EMAIL
            })
            .times(1)
            // returns connection error
            .returning(move |_, _, _| Box::pin(async move { Err(VerifyError::Connection) }));
        // save latest state should not be called on token verification error
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .never();
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::TokenRepoError(VerifyError::Connection)
        );
    }
    #[rstest]
    async fn test_verify_fail_token_verification_not_found(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| {
                Box::pin({
                    let record = verification_email_sent_record.clone();
                    async move { Ok(record) }
                })
            });
        dependency_provider
            .db
            .token_repo
            .expect_verify()
            // makes sure the correct token is used
            .withf(move |_, actual_email, actual_token| {
                actual_token == TEST_TOKEN && actual_email == TEST_EMAIL
            })
            .times(1)
            // returns connection error
            .returning(move |_, _, _| Box::pin(async move { Err(VerifyError::NotFound) }));
        // save latest state should not be called on token verification error
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .never();
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::TokenRepoError(VerifyError::NotFound)
        );
    }
    #[rstest]
    async fn test_verify_fail_token_verification_token_missmatch(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let wrong_token = "wrong_token".to_string();
        let req = Request {
            id: signup_id,
            token: wrong_token.clone(),
        };
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| {
                Box::pin({
                    let record = verification_email_sent_record.clone();
                    async move { Ok(record) }
                })
            });
        dependency_provider
            .db
            .token_repo
            .expect_verify()
            // makes sure the correct token is used
            .withf(move |_, actual_email, actual_token| {
                actual_token == wrong_token.clone() && actual_email == TEST_EMAIL
            })
            .times(1)
            // returns connection error
            .returning(move |_, _, _| Box::pin(async move { Err(VerifyError::Mismatch) }));
        // save latest state should not be called on token verification error
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .never();
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::TokenRepoError(VerifyError::Mismatch)
        );
    }
    #[rstest]
    async fn test_verify_fail_token_verification_token_expired(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        let process: SignupProcess<VerificationEmailSent> =
            verification_email_sent_record.clone().try_into().unwrap();
        // record to be passed to the save latest state method
        let record_to_save = process.fail(SignupError::VerificationTimedOut).into();
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| {
                Box::pin({
                    let record = verification_email_sent_record.clone();
                    async move { Ok(record) }
                })
            });
        dependency_provider
            .db
            .token_repo
            .expect_verify()
            // makes sure the correct token is used
            .withf(move |_, actual_email, actual_token| {
                actual_token == TEST_TOKEN && actual_email == TEST_EMAIL
            })
            .times(1)
            // returns connection error
            .returning(move |_, _, _| Box::pin(async move { Err(VerifyError::TokenExpired) }));
        // save latest state should be called for the failed verification
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(move |_, _| Box::pin(async move { Ok(()) }));
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution error
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::TokenRepoError(VerifyError::TokenExpired)
        );
    }
    #[rstest]
    async fn test_verify_email_fail_save_latest_state_connection(
        mut dependency_provider: MockDependencyProvider,
        signup_id: SignupId,
        verification_email_sent_record: SignupProcessRepoRecord,
    ) {
        // fixtures
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        let process: SignupProcess<VerificationEmailSent> =
            verification_email_sent_record.clone().try_into().unwrap();
        // record to be passed to the save latest state method
        let record_to_save = process.verify_email().into();
        // Mock setup -- predicates and return values
        dependency_provider
            .db
            .signup_process_repo
            .expect_get_latest_state()
            // makes sure the correct id is used
            .withf(move |_, actual_id| actual_id == &signup_id)
            .times(1)
            // returns the record with the correct state
            .returning(move |_, _| {
                Box::pin({
                    let record = verification_email_sent_record.clone();
                    async move { Ok(record) }
                })
            });
        dependency_provider
            .db
            .token_repo
            .expect_verify()
            // makes sure the correct token is used
            .withf(move |_, actual_email, actual_token| {
                actual_token == TEST_TOKEN && actual_email == TEST_EMAIL
            })
            .times(1)
            // returns Ok
            .returning(move |_, _, _| Box::pin(async move { Ok(()) }));
        dependency_provider
            .db
            .signup_process_repo
            .expect_save_latest_state()
            .withf(move |_, actual_record| actual_record == &record_to_save)
            .times(1)
            .returning(move |_, _| Box::pin(async move { Err(SaveError::Connection) }));
        // Usecase Initialization
        let usecase = <VerifyEmail<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
            &dependency_provider,
        );
        // Usecase Execution -- mock predicates will fail during execution
        let result = usecase.exec(req).await;
        // Assert execution success
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), Error::Repo);
    }
    #[rstest]
    fn test_authorize_admin_zero(signup_id: SignupId, auth_context_admin: AuthContext) {
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        let result = VerifyEmail::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_admin));
        assert!(result.is_ok());
    }

    #[rstest]
    fn test_authorize_user_zero(signup_id: SignupId, auth_context_user: AuthContext) {
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        let result = VerifyEmail::new(&MockDependencyProvider::default())
            .authorize(&req, Some(auth_context_user));
        assert!(result.is_ok());
    }
    #[rstest]
    fn test_authorize_none(signup_id: SignupId) {
        let req = Request {
            id: signup_id,
            token: TEST_TOKEN.to_string(),
        };
        let auth_context = None;
        let result =
            VerifyEmail::new(&MockDependencyProvider::default()).authorize(&req, auth_context);
        assert!(result.is_ok());
    }
}
