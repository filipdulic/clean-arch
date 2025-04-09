use std::sync::Arc;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct UserParseIdError; // map this to proper error type

use ca_application::gateway::repository::{
    signup_process::Record as SignupProcessRecord, user::Record as UserRecord,
};
use ca_domain::entity::{
    signup_process::SignupStateEnum as EntitySignupStateEnum,
    user::{self, Password},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SignupStateFailedError {
    TokenGenrationFailed,
    VerificationEmailSendError,
    VerificationTimedOut,
    CompletionTimedOut,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignupStateEnum {
    Initialized {
        email: String,
    },
    VerificationEmailSent {
        email: String,
    },
    EmailVerified {
        email: String,
    },
    Completed {
        email: String,
        username: String,
        password: String,
    },
    ForDeletion,
    Failed {
        previous_state: Arc<SignupStateEnum>,
        error: SignupStateFailedError,
    },
}
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub role: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupProcess {
    pub signup_process_id: String,
    pub state: SignupStateEnum,
    pub entered_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationToken {
    pub token: uuid::Uuid,
    pub created_at: DateTime<Utc>,
}

impl From<&SignupStateFailedError> for ca_domain::entity::signup_process::Error {
    fn from(value: &SignupStateFailedError) -> ca_domain::entity::signup_process::Error {
        match value {
            SignupStateFailedError::TokenGenrationFailed => {
                ca_domain::entity::signup_process::Error::TokenGenrationFailed
            }
            SignupStateFailedError::VerificationEmailSendError => {
                ca_domain::entity::signup_process::Error::VerificationEmailSendError
            }
            SignupStateFailedError::VerificationTimedOut => {
                ca_domain::entity::signup_process::Error::VerificationTimedOut
            }
            SignupStateFailedError::CompletionTimedOut => {
                ca_domain::entity::signup_process::Error::CompletionTimedOut
            }
        }
    }
}

impl From<ca_domain::entity::signup_process::Error> for SignupStateFailedError {
    fn from(value: ca_domain::entity::signup_process::Error) -> SignupStateFailedError {
        match value {
            ca_domain::entity::signup_process::Error::TokenGenrationFailed => {
                SignupStateFailedError::TokenGenrationFailed
            }
            ca_domain::entity::signup_process::Error::VerificationEmailSendError => {
                SignupStateFailedError::VerificationEmailSendError
            }
            ca_domain::entity::signup_process::Error::VerificationTimedOut => {
                SignupStateFailedError::VerificationTimedOut
            }
            ca_domain::entity::signup_process::Error::CompletionTimedOut => {
                SignupStateFailedError::CompletionTimedOut
            }
        }
    }
}

impl From<EntitySignupStateEnum> for SignupStateEnum {
    fn from(value: EntitySignupStateEnum) -> SignupStateEnum {
        match value {
            EntitySignupStateEnum::Initialized { email } => SignupStateEnum::Initialized {
                email: email.to_string(),
            },
            EntitySignupStateEnum::VerificationEmailSent { email } => {
                SignupStateEnum::VerificationEmailSent {
                    email: email.to_string(),
                }
            }
            EntitySignupStateEnum::EmailVerified { email } => SignupStateEnum::EmailVerified {
                email: email.to_string(),
            },
            EntitySignupStateEnum::Completed {
                email,
                username,
                password,
            } => SignupStateEnum::Completed {
                email: email.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            },
            EntitySignupStateEnum::ForDeletion => SignupStateEnum::ForDeletion,
            EntitySignupStateEnum::Failed {
                previous_state,
                error,
            } => SignupStateEnum::Failed {
                previous_state: Arc::new(previous_state.as_ref().clone().into()),
                error: error.into(),
            },
        }
    }
}
impl From<SignupProcessRecord> for SignupProcess {
    fn from(value: SignupProcessRecord) -> SignupProcess {
        SignupProcess {
            signup_process_id: value.id.to_string(),
            state: value.state.into(),
            entered_at: value.entered_at,
        }
    }
}

impl From<&SignupStateEnum> for EntitySignupStateEnum {
    fn from(value: &SignupStateEnum) -> EntitySignupStateEnum {
        match value {
            SignupStateEnum::Initialized { email } => EntitySignupStateEnum::Initialized {
                email: user::Email::new(email),
            },
            SignupStateEnum::VerificationEmailSent { email } => {
                EntitySignupStateEnum::VerificationEmailSent {
                    email: user::Email::new(email),
                }
            }
            SignupStateEnum::EmailVerified { email } => EntitySignupStateEnum::EmailVerified {
                email: user::Email::new(email),
            },
            SignupStateEnum::Completed {
                email,
                username,
                password,
            } => EntitySignupStateEnum::Completed {
                email: user::Email::new(email),
                username: user::UserName::new(username.clone()),
                password: user::Password::new(password.clone()),
            },
            SignupStateEnum::ForDeletion => EntitySignupStateEnum::ForDeletion,
            SignupStateEnum::Failed {
                previous_state,
                error,
            } => EntitySignupStateEnum::Failed {
                previous_state: Arc::new(previous_state.as_ref().into()),
                error: error.into(),
            },
        }
    }
}

impl From<SignupStateEnum> for EntitySignupStateEnum {
    fn from(value: SignupStateEnum) -> EntitySignupStateEnum {
        (&value).into()
    }
}

impl From<SignupProcess> for SignupProcessRecord {
    fn from(value: SignupProcess) -> SignupProcessRecord {
        let id = match value.signup_process_id.parse::<Uuid>() {
            Ok(id) => id.into(),
            Err(_) => unreachable!(),
        };
        SignupProcessRecord {
            id,
            state: value.state.into(),
            entered_at: value.entered_at,
        }
    }
}

impl From<UserRecord> for User {
    fn from(value: UserRecord) -> User {
        User {
            user_id: value.user.id().to_string(),
            role: value.user.role().to_string(),
            email: value.user.email().to_string(),
            username: value.user.username().to_string(),
            password: value.user.password().to_string(),
        }
    }
}
impl TryInto<UserRecord> for &User {
    type Error = UserParseIdError;
    fn try_into(self) -> Result<UserRecord, Self::Error> {
        let id = self
            .user_id
            .parse::<Uuid>()
            .map_err(|_| UserParseIdError)?
            .into();
        let username = user::UserName::new(self.username.clone());
        let email = user::Email::new(&self.email);
        let password = Password::new(self.password.clone());
        // TODO: handle invalid role from db.
        let role = self
            .role
            .parse()
            .unwrap_or(ca_domain::value_object::Role::User);
        Ok(UserRecord {
            user: user::User::new(id, role, email, username, password),
        })
    }
}
impl TryInto<UserRecord> for User {
    type Error = UserParseIdError;
    fn try_into(self) -> Result<UserRecord, Self::Error> {
        let user = &self;
        user.try_into()
    }
}
