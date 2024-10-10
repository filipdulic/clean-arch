use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapter::model::app::user::ParseIdError as UserParseIdError,
    application::gateway::repository::{
        signup_process::Record as SignupProcessRecord, user::Record as UserRecord,
    },
    domain::entity::{signup_process::SignupStateEnum as EntitySignupStateEnum, user},
};

#[derive(Debug, Serialize, Deserialize)]
pub enum SignupStateEnum {
    Initialized { username: String },
    EmailAdded { username: String, email: String },
    Completed { username: String, email: String },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupProcess {
    pub signup_process_id: String,
    pub state: SignupStateEnum,
}

impl From<EntitySignupStateEnum> for SignupStateEnum {
    fn from(value: EntitySignupStateEnum) -> SignupStateEnum {
        match value {
            EntitySignupStateEnum::Initialized { username } => SignupStateEnum::Initialized {
                username: username.to_string(),
            },
            EntitySignupStateEnum::EmailAdded { username, email } => SignupStateEnum::EmailAdded {
                username: username.to_string(),
                email: email.to_string(),
            },
            EntitySignupStateEnum::Completed { username, email } => SignupStateEnum::Completed {
                username: username.to_string(),
                email: email.to_string(),
            },
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
            SignupStateEnum::Initialized { username } => EntitySignupStateEnum::Initialized {
                username: user::UserName::new(username.clone()),
            },
            SignupStateEnum::EmailAdded { username, email } => EntitySignupStateEnum::EmailAdded {
                username: user::UserName::new(username.clone()),
                email: user::Email::new(email.clone()),
            },
            SignupStateEnum::Completed { username, email } => EntitySignupStateEnum::Completed {
                username: user::UserName::new(username.clone()),
                email: user::Email::new(email.clone()),
            },
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
            username: value.user.username().to_string(),
            email: value.user.email().to_string(),
        }
    }
}

impl TryInto<UserRecord> for User {
    type Error = UserParseIdError;
    fn try_into(self) -> Result<UserRecord, Self::Error> {
        let id = self
            .user_id
            .parse::<Uuid>()
            .map_err(|_| UserParseIdError)?
            .into();
        let username = user::UserName::new(self.username);
        let email = user::Email::new(self.email);
        Ok(UserRecord {
            user: user::User::new(id, username, email),
        })
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
        Ok(UserRecord {
            user: user::User::new(id, username, email),
        })
    }
}
