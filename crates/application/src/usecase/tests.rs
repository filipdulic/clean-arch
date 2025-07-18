pub mod fixtures {
    use std::{str::FromStr, sync::Arc};

    use ca_domain::{
        entity::{
            auth_context::AuthContext,
            signup_process::{Error as SignupError, Id as SignupId, SignupStateEnum},
            user::{Email, Id as UserId, User},
        },
        value_object::{Password, Role, UserName},
    };
    use rstest::*;

    use crate::gateway::{
        database::{
            signup_process::Record as SignupProcessRepoRecord, token::Record as TokenRepoRecord,
            user::Record as UserRecord,
        },
        mock::MockDependencyProvider,
    };

    pub static TEST_EMAIL: &str = "test@email.com";
    pub static TEST_TOKEN: &str = "test_token";
    pub static TEST_UUID: &str = "9dcccf0f-a1ff-49fb-a238-cd9d88502391";
    pub static TEST_UUID2: &str = "03b85a20-e4cb-4e34-b6a5-a8cd86ba4a98";
    pub static TEST_USERNAME: &str = "test_username";
    pub static TEST_PASSWORD: &str = "test_password";

    #[fixture]
    pub fn signup_id() -> SignupId {
        SignupId::new(uuid::Uuid::from_str(TEST_UUID).unwrap())
    }
    #[fixture]
    pub fn user_id_zero() -> UserId {
        UserId::new(uuid::Uuid::nil())
    }
    #[fixture]
    pub fn auth_context_admin(user_id_zero: UserId) -> AuthContext {
        AuthContext::new(user_id_zero, Role::Admin)
    }
    #[fixture]
    pub fn auth_context_user(user_id_zero: UserId) -> AuthContext {
        AuthContext::new(user_id_zero, Role::User)
    }
    #[fixture]
    pub fn email() -> Email {
        Email::new(TEST_EMAIL)
    }
    #[fixture]
    pub fn token_repo_record() -> TokenRepoRecord {
        TokenRepoRecord {
            token: TEST_TOKEN.to_string(),
        }
    }
    #[fixture]
    pub fn initialized_state(email: Email) -> SignupStateEnum {
        SignupStateEnum::Initialized { email }
    }
    #[fixture]
    pub fn dependency_provider() -> MockDependencyProvider {
        MockDependencyProvider::default()
    }
    #[fixture]
    pub fn initialized_record(signup_id: SignupId, email: Email) -> SignupProcessRepoRecord {
        SignupProcessRepoRecord {
            id: signup_id,
            state: SignupStateEnum::Initialized { email },
            entered_at: chrono::Utc::now(),
        }
    }
    #[fixture]
    pub fn verification_email_sent_record(
        signup_id: SignupId,
        email: Email,
    ) -> SignupProcessRepoRecord {
        SignupProcessRepoRecord {
            id: signup_id,
            state: SignupStateEnum::VerificationEmailSent { email },
            entered_at: chrono::Utc::now(),
        }
    }
    #[fixture]
    pub fn email_verified_record(signup_id: SignupId, email: Email) -> SignupProcessRepoRecord {
        SignupProcessRepoRecord {
            id: signup_id,
            state: SignupStateEnum::EmailVerified { email },
            entered_at: chrono::Utc::now(),
        }
    }
    #[fixture]
    pub fn failed_verification_email_sent_record(
        signup_id: SignupId,
        email: Email,
    ) -> SignupProcessRepoRecord {
        SignupProcessRepoRecord {
            id: signup_id,
            state: SignupStateEnum::Failed {
                previous_state: Arc::new(SignupStateEnum::VerificationEmailSent { email }),
                error: SignupError::VerificationTimedOut,
            },
            entered_at: chrono::Utc::now(),
        }
    }
    #[fixture]
    pub fn failed_verification_email_verified_record(
        signup_id: SignupId,
        email: Email,
    ) -> SignupProcessRepoRecord {
        SignupProcessRepoRecord {
            id: signup_id,
            state: SignupStateEnum::Failed {
                previous_state: Arc::new(SignupStateEnum::EmailVerified { email }),
                error: SignupError::VerificationTimedOut,
            },
            entered_at: chrono::Utc::now(),
        }
    }
    #[fixture]
    pub fn failed_initialized_record(signup_id: SignupId, email: Email) -> SignupProcessRepoRecord {
        SignupProcessRepoRecord {
            id: signup_id,
            state: SignupStateEnum::Failed {
                previous_state: Arc::new(SignupStateEnum::Initialized { email }),
                error: SignupError::TokenGenrationFailed,
            },
            entered_at: chrono::Utc::now(),
        }
    }

    #[fixture]
    pub fn state_chain_record_vector(
        signup_id: SignupId,
        email: Email,
    ) -> Vec<SignupProcessRepoRecord> {
        vec![
            SignupProcessRepoRecord {
                id: signup_id,
                state: SignupStateEnum::Initialized {
                    email: email.clone(),
                },
                entered_at: chrono::Utc::now(),
            },
            SignupProcessRepoRecord {
                id: signup_id,
                state: SignupStateEnum::VerificationEmailSent {
                    email: email.clone(),
                },
                entered_at: chrono::Utc::now(),
            },
            SignupProcessRepoRecord {
                id: signup_id,
                state: SignupStateEnum::EmailVerified { email },
                entered_at: chrono::Utc::now(),
            },
        ]
    }
    #[fixture]
    pub fn user_record() -> UserRecord {
        UserRecord {
            user: User::new(
                UserId::new(uuid::Uuid::parse_str(TEST_UUID).unwrap()),
                Role::User,
                Email::new(TEST_EMAIL),
                UserName::new(TEST_USERNAME),
                Password::new(TEST_PASSWORD),
            ),
        }
    }
    #[fixture]
    pub fn user_records() -> Vec<UserRecord> {
        vec![
            UserRecord {
                user: User::new(
                    UserId::new(uuid::Uuid::parse_str(TEST_UUID).unwrap()),
                    Role::User,
                    Email::new(TEST_EMAIL),
                    UserName::new(TEST_USERNAME),
                    Password::new(TEST_PASSWORD),
                ),
            },
            UserRecord {
                user: User::new(
                    UserId::new(uuid::Uuid::parse_str(TEST_UUID2).unwrap()),
                    Role::User,
                    Email::new(TEST_EMAIL),
                    UserName::new(TEST_USERNAME),
                    Password::new(TEST_PASSWORD),
                ),
            },
        ]
    }
    #[fixture]
    pub fn user_id() -> UserId {
        UserId::new(uuid::Uuid::from_str(TEST_UUID).unwrap())
    }
}
