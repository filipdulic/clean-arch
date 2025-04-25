use crate::{
    gateway::{
        database::{
            signup_process::{GetError, SaveError},
            user::{self, SaveError as UserSaveError},
        },
        DatabaseProvider,
    },
    usecase::Usecase,
};

use ca_domain::{
    entity::{
        auth_strategy::AuthStrategy,
        signup_process::{EmailVerified, Id, SignupProcess},
        user::{Password, User, UserName},
    },
    value_object::Role,
};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Request {
    pub id: Id,
    #[validate(length(min = 1, max = 30))]
    pub username: String,
    #[validate(length(min = 5, max = 60))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Response {
    pub record: user::Record,
}
pub struct Complete<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
    #[error("SignupProcess {0} in incorrect state")]
    IncorrectState(Id),
    #[error("SignupProcess completion timed out")]
    CompletionTimedOut,
    #[error(transparent)]
    Validation(#[from] validator::ValidationErrors),
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
    fn from(e: SaveError) -> Self {
        match e {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<UserSaveError> for Error {
    fn from(e: UserSaveError) -> Self {
        match e {
            UserSaveError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for Complete<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        // Validate the request
        req.validate()?;
        let transaction = self
            .dependency_provider
            .database()
            .begin_transaction()
            .await;
        let record = self
            .dependency_provider
            .database()
            .signup_process_repo()
            .get_latest_state(None, req.id)
            .await
            .map_err(|e| (e, req.id))?;
        let process: SignupProcess<EmailVerified> = record.try_into().map_err(|e| (e, req.id))?;
        if Utc::now() - Duration::days(1) > process.entered_at() {
            let process =
                process.fail(ca_domain::entity::signup_process::Error::CompletionTimedOut);
            self.dependency_provider
                .database()
                .signup_process_repo()
                .save_latest_state(None, process.into())
                .await?;
            self.dependency_provider
                .database()
                .commit_transaction(transaction)
                .await
                .map_err(|_| SaveError::Connection)?;
            return Err(Self::Error::CompletionTimedOut);
        }
        let username = UserName::new(req.username);
        let password = Password::new(req.password);
        let process = process.complete(username, password);
        let user: User = User::new(
            ca_domain::entity::user::Id::new(req.id),
            Role::User,
            process.email(),
            process.username(),
            process.password(),
        );
        // Save User first, then save SignupProcess
        self.dependency_provider
            .database()
            .user_repo()
            .save(None, user.clone().into())
            .await?;
        // if save user fails, we should not save the signup process
        self.dependency_provider
            .database()
            .signup_process_repo()
            .save_latest_state(None, process.clone().into())
            .await?;
        self.dependency_provider
            .database()
            .commit_transaction(transaction)
            .await
            .map_err(|_| SaveError::Connection)?;
        Ok(Self::Response {
            record: user.into(),
        })
    }

    fn new(db: &'d D) -> Self {
        Self {
            dependency_provider: db,
        }
    }
    fn auth_strategy(&self) -> AuthStrategy {
        AuthStrategy::Public
    }
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;

//     use super::*;
//     use crate::gateway::{
//         database::{
//             mock::{MockDatabase, MockSignupIdGen},
//             signup_process::{
//                 MockRepo as MockSignupProcessRepo, Record as SignupProcessRepoRecord,
//             },
//             token::MockRepo as MockTokenRepo,
//             user::MockRepo as MockUserRepo,
//         },
//         mock::MockDependencyProvider,
//         service::{auth::mock::MockAuthPacker, email::mock::MockEmailVerificationService},
//     };
//     use ca_domain::{
//         entity::{
//             auth_context::AuthContext,
//             signup_process::{Error as SignupError, Id as SignupId},
//         },
//         value_object::Email,
//     };
//     use rstest::*;

//     #[rstest]
//     async fn test_complete_success(
//         signup_id: SignupId,
//         email_verified_record: SignupProcessRepoRecord,
//     ) {
//         // fixtures
//         let req = Request {
//             id: signup_id,
//             username: TEST_USERNAME.to_string(),
//             password: TEST_PASSWORD.to_string(),
//         };
//         let process: SignupProcess<EmailVerified> =
//             email_verified_record.clone().try_into().unwrap();
//         // record to be passed to the save latest state method
//         let record_to_save = process
//             .complete(UserName::new(TEST_USERNAME), Password::new(TEST_PASSWORD))
//             .into();
//         let user: User = User::new(
//             ca_domain::entity::user::Id::new(signup_id),
//             Role::User,
//             Email::new(TEST_EMAIL),
//             UserName::new(TEST_USERNAME),
//             Password::new(TEST_PASSWORD),
//         );
//         // Mock setup -- predicates and return values
//         let mut signup_process_repo = MockSignupProcessRepo::new();
//         signup_process_repo
//             .expect_get_latest_state()
//             // makes sure the correct id is used
//             .withf(move |_, actual_id| actual_id == &signup_id)
//             .times(1)
//             // returns the record with the correct state
//             .returning(move |_, _| Ok(email_verified_record.clone()));
//         let mut user_repo = MockUserRepo::new();
//         user_repo
//             .expect_save()
//             // makes sure the correct user is used
//             .withf({
//                 let user = user.clone();
//                 move |_, actual_user| actual_user.user == user
//             })
//             .times(1)
//             // returns Ok
//             .returning(move |_, _| Ok(()));
//         signup_process_repo
//             .expect_save_latest_state()
//             .withf(move |_, actual_record| actual_record == &record_to_save)
//             .times(1)
//             .returning(move |_, _| Ok(()));
//         // Usecase Initialization
//         let dependency_provider = MockDependencyProvider {
//             db: Arc::new(MockDatabase::new(
//                 signup_process_repo,
//                 MockSignupIdGen::new(),
//                 MockTokenRepo::new(),
//                 user_repo,
//             )),
//             email_verification_service: Arc::new(MockEmailVerificationService::new()),
//             auth_packer: Arc::new(MockAuthPacker::new()),
//         };
//         let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//             &dependency_provider,
//         );
//         // Usecase Execution -- mock predicates will fail during execution
//         let result = usecase.exec(req).await;
//         // Assert execution success
//         assert!(result.is_ok());
//         let response = result.unwrap();
//         assert_eq!(response.record.user, user);
//     }
// #[rstest]
// async fn test_complete_fail_request_validation(
//     mut dependency_provider: MockDependencyProvider,
//     signup_id: SignupId,
// ) {
//     // fixtures
//     let req = Request {
//         id: signup_id,
//         username: "".to_string(),
//         password: "".to_string(),
//     };
//     // Mock setup -- predicates and return values
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_get_latest_state()
//         // makes sure get_latest_state is never called
//         .never();
//     // Usecase Initialization
//     let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//         &dependency_provider,
//     );
//     // Usecase Execution -- mock predicates will fail during execution
//     let result = usecase.exec(req).await;
//     // Assert execution success
//     assert!(result.is_err());
//     let error_string = result.unwrap_err().to_string();
//     assert!(error_string.contains("password: Validation error: length"));
//     assert!(error_string.contains("username: Validation error: length"));
// }
// #[rstest]
// async fn test_complete_fail_get_latest_state_connection(
//     mut dependency_provider: MockDependencyProvider,
//     signup_id: SignupId,
// ) {
//     // fixtures
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     // Mock setup -- predicates and return values
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_get_latest_state()
//         // makes sure the correct id is used
//         .withf(move |_, actual_id| actual_id == &signup_id)
//         .times(1)
//         // returns a connection error
//         .returning(move |_, _| Box::pin(async move { Err(GetError::Connection) }));
//     // Usecase Initialization
//     let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//         &dependency_provider,
//     );
//     // Usecase Execution -- mock predicates will fail during execution
//     let result = usecase.exec(req).await;
//     // Assert execution error
//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), Error::Repo);
// }
// #[rstest]
// async fn test_complete_fail_get_latest_state_not_found(
//     mut dependency_provider: MockDependencyProvider,
//     signup_id: SignupId,
// ) {
//     // fixtures
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     // Mock setup -- predicates and return values
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_get_latest_state()
//         // makes sure the correct id is used
//         .withf(move |_, actual_id| actual_id == &signup_id)
//         .times(1)
//         // returns a not found error
//         .returning(move |_, _| Box::pin(async move { Err(GetError::NotFound) }));
//     // Usecase Initialization
//     let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//         &dependency_provider,
//     );
//     // Usecase Execution -- mock predicates will fail during execution
//     let result = usecase.exec(req).await;
//     // Assert execution error
//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), Error::NotFound(signup_id));
// }
// #[rstest]
// async fn test_complete_fail_get_latest_state_incorrect_state(
//     mut dependency_provider: MockDependencyProvider,
//     signup_id: SignupId,
//     initialized_record: SignupProcessRepoRecord,
// ) {
//     // fixtures
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     // Mock setup -- predicates and return values
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_get_latest_state()
//         // makes sure the correct id is used
//         .withf(move |_, actual_id| actual_id == &signup_id)
//         .times(1)
//         // returns the record with the incorrect state
//         .returning(move |_, _| {
//             let record = initialized_record.clone();
//             Box::pin(async move { Ok(record) })
//         });
//     // Usecase Initialization
//     let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//         &dependency_provider,
//     );
//     // Usecase Execution -- mock predicates will fail during execution
//     let result = usecase.exec(req).await;
//     // Assert execution errpr
//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), Error::IncorrectState(signup_id));
// }
// #[rstest]
// async fn test_complete_fail_completion_time_expired(
//     mut dependency_provider: MockDependencyProvider,
//     signup_id: SignupId,
//     mut email_verified_record: SignupProcessRepoRecord,
// ) {
//     // fixtures
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     email_verified_record.entered_at = Utc::now() - Duration::days(2);
//     let process: SignupProcess<EmailVerified> =
//         email_verified_record.clone().try_into().unwrap();
//     // record to be passed to the save latest state method
//     let record_to_save = process.fail(SignupError::CompletionTimedOut).into();
//     // Mock setup -- predicates and return values
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_get_latest_state()
//         // makes sure the correct id is used
//         .withf(move |_, actual_id| actual_id == &signup_id)
//         .times(1)
//         // returns the record with the incorrect state
//         .returning(move |_, _| {
//             let record = email_verified_record.clone();
//             Box::pin(async move { Ok(record) })
//         });
//     dependency_provider
//         .db
//         .user_repo
//         .expect_save()
//         // makes sure save user is never called
//         .never();
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_save_latest_state()
//         .withf(move |_, actual_record| actual_record == &record_to_save)
//         .times(1)
//         .returning(move |_, _| Box::pin(async move { Ok(()) }));
//     // Usecase Initialization
//     let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//         &dependency_provider,
//     );
//     // Usecase Execution -- mock predicates will fail during execution
//     let result = usecase.exec(req).await;
//     // Assert execution errpr
//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), Error::CompletionTimedOut);
// }
// #[rstest]
// async fn test_complete_fail_user_repo_connection(
//     mut dependency_provider: MockDependencyProvider,
//     signup_id: SignupId,
//     email_verified_record: SignupProcessRepoRecord,
// ) {
//     // fixtures
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     let user: User = User::new(
//         ca_domain::entity::user::Id::new(signup_id),
//         Role::User,
//         Email::new(TEST_EMAIL),
//         UserName::new(TEST_USERNAME),
//         Password::new(TEST_PASSWORD),
//     );
//     // Mock setup -- predicates and return values
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_get_latest_state()
//         // makes sure the correct id is used
//         .withf(move |_, actual_id| actual_id == &signup_id)
//         .times(1)
//         // returns the record with the incorrect state
//         .returning(move |_, _| {
//             let record = email_verified_record.clone();
//             Box::pin(async move { Ok(record) })
//         });
//     dependency_provider
//         .db
//         .user_repo
//         .expect_save()
//         // makes sure the correct user is used
//         .withf({
//             let user = user.clone();
//             move |_, actual_user| actual_user.user == user
//         })
//         .times(1)
//         // returns Ok
//         .returning(move |_, _| Box::pin(async move { Err(UserSaveError::Connection) }));
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_save_latest_state()
//         .never();
//     // Usecase Initialization
//     let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//         &dependency_provider,
//     );

//     // Usecase Execution -- mock predicates will fail during execution
//     let result = usecase.exec(req).await;
//     // Assert execution errpr
//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), Error::Repo);
// }
// #[rstest]
// async fn test_complete_fail_save_latest_state_connection(
//     mut dependency_provider: MockDependencyProvider,
//     signup_id: SignupId,
//     email_verified_record: SignupProcessRepoRecord,
// ) {
//     // fixtures
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     let process: SignupProcess<EmailVerified> =
//         email_verified_record.clone().try_into().unwrap();
//     // record to be passed to the save latest state method
//     let record_to_save = process
//         .complete(UserName::new(TEST_USERNAME), Password::new(TEST_PASSWORD))
//         .into();
//     let user: User = User::new(
//         ca_domain::entity::user::Id::new(signup_id),
//         Role::User,
//         Email::new(TEST_EMAIL),
//         UserName::new(TEST_USERNAME),
//         Password::new(TEST_PASSWORD),
//     );
//     // Mock setup -- predicates and return values
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_get_latest_state()
//         // makes sure the correct id is used
//         .withf(move |_, actual_id| actual_id == &signup_id)
//         .times(1)
//         // returns the record with the incorrect state
//         .returning(move |_, _| {
//             let record = email_verified_record.clone();
//             Box::pin(async move { Ok(record) })
//         });
//     dependency_provider
//         .db
//         .user_repo
//         .expect_save()
//         // makes sure the correct user is used
//         .withf({
//             let user = user.clone();
//             move |_, actual_user| actual_user.user == user
//         })
//         .times(1)
//         // returns Ok
//         .returning(move |_, _| Box::pin(async move { Ok(()) }));
//     dependency_provider
//         .db
//         .signup_process_repo
//         .expect_save_latest_state()
//         .withf(move |_, actual_record| actual_record == &record_to_save)
//         .times(1)
//         .returning(move |_, _| Box::pin(async move { Err(SaveError::Connection) }));
//     // Usecase Initialization
//     let usecase = <Complete<MockDependencyProvider> as Usecase<MockDependencyProvider>>::new(
//         &dependency_provider,
//     );

//     // Usecase Execution -- mock predicates will fail during execution
//     let result = usecase.exec(req).await;
//     // Assert execution errpr
//     assert!(result.is_err());
//     assert_eq!(result.unwrap_err(), Error::Repo);
// }
// #[rstest]
// fn test_authorize_admin_zero(auth_context_admin: AuthContext, signup_id: SignupId) {
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     let result = Complete::new(&MockDependencyProvider::default())
//         .authorize(&req, Some(auth_context_admin));
//     assert!(result.is_ok());
// }

// #[rstest]
// fn test_authorize_user_zero(auth_context_user: AuthContext, signup_id: SignupId) {
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     let result = Complete::new(&MockDependencyProvider::default())
//         .authorize(&req, Some(auth_context_user));
//     assert!(result.is_ok());
// }
// #[rstest]
// fn test_authorize_none(signup_id: SignupId) {
//     let req = Request {
//         id: signup_id,
//         username: TEST_USERNAME.to_string(),
//         password: TEST_PASSWORD.to_string(),
//     };
//     let result = Complete::new(&MockDependencyProvider::default()).authorize(&req, None);
//     assert!(result.is_ok());
// }
// }
