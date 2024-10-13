use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapter::model::app::user::ParseIdError as UserParseIdError,
    application::gateway::repository::{
        signup_process::Record as SignupProcessRecord, user::Record as UserRecord,
    },
    domain::entity::{
        signup_process::SignupStateEnum as EntitySignupStateEnum,
        user::{self, Password},
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub enum SignupStateEnum {
    Initialized {
        email: String,
    },
    EmailVerified {
        email: String,
    },
    VerificationTimedOut {
        email: String,
    },
    CompletionTimedOut {
        email: String,
    },
    Completed {
        email: String,
        username: String,
        password: String,
    },
    ForDeletion,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupProcess {
    pub signup_process_id: String,
    pub state: SignupStateEnum,
}

impl From<EntitySignupStateEnum> for SignupStateEnum {
    fn from(value: EntitySignupStateEnum) -> SignupStateEnum {
        match value {
            EntitySignupStateEnum::Initialized { email } => SignupStateEnum::Initialized {
                email: email.to_string(),
            },
            EntitySignupStateEnum::EmailVerified { email } => SignupStateEnum::EmailVerified {
                email: email.to_string(),
            },
            EntitySignupStateEnum::VerificationTimedOut { email } => {
                SignupStateEnum::VerificationTimedOut {
                    email: email.to_string(),
                }
            }
            EntitySignupStateEnum::CompletionTimedOut { email } => {
                SignupStateEnum::CompletionTimedOut {
                    email: email.to_string(),
                }
            }
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
        }
    }
}
impl From<SignupProcessRecord> for SignupProcess {
    fn from(value: SignupProcessRecord) -> SignupProcess {
        SignupProcess {
            signup_process_id: value.id.to_string(),
            state: value.state.into(),
        }
    }
}

impl From<SignupStateEnum> for EntitySignupStateEnum {
    fn from(value: SignupStateEnum) -> EntitySignupStateEnum {
        match value {
            SignupStateEnum::Initialized { email } => EntitySignupStateEnum::Initialized {
                email: user::Email::new(email.clone()),
            },
            SignupStateEnum::EmailVerified { email } => EntitySignupStateEnum::EmailVerified {
                email: user::Email::new(email.clone()),
            },
            SignupStateEnum::VerificationTimedOut { email } => {
                EntitySignupStateEnum::VerificationTimedOut {
                    email: user::Email::new(email.clone()),
                }
            }
            SignupStateEnum::CompletionTimedOut { email } => {
                EntitySignupStateEnum::CompletionTimedOut {
                    email: user::Email::new(email.clone()),
                }
            }
            SignupStateEnum::Completed {
                email,
                username,
                password,
            } => EntitySignupStateEnum::Completed {
                email: user::Email::new(email.clone()),
                username: user::UserName::new(username.clone()),
                password: user::Password::new(password.clone()),
            },
            SignupStateEnum::ForDeletion => EntitySignupStateEnum::ForDeletion,
        }
    }
}

impl From<SignupProcess> for SignupProcessRecord {
    fn from(value: SignupProcess) -> SignupProcessRecord {
        SignupProcessRecord {
            id: value.signup_process_id.parse::<Uuid>().unwrap().into(),
            state: value.state.into(),
        }
    }
}

impl From<UserRecord> for User {
    fn from(value: UserRecord) -> User {
        User {
            user_id: value.user.id().to_string(),
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
        let email = user::Email::new(self.email.clone());
        let password = Password::new(self.password.clone());
        Ok(UserRecord {
            user: user::User::new(id, email, username, password),
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
