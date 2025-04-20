pub mod fixtures {
    use std::str::FromStr;

    use ca_domain::{
        entity::{
            auth_context::AuthContext,
            signup_process::{Id as SignupId, SignupStateEnum},
            user::{Email, Id as UserId},
        },
        value_object::Role,
    };
    use rstest::*;

    use crate::gateway::{
        database::{
            signup_process::Record as SignupProcessRepoRecord, token::Record as TokenRepoRecord,
        },
        mock::MockDependencyProvider,
    };

    pub static TEST_EMAIL: &str = "test@email.com";
    pub static TEST_TOKEN: &str = "test_token";
    pub static TEST_UUID: &str = "9dcccf0f-a1ff-49fb-a238-cd9d88502391";

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
}
