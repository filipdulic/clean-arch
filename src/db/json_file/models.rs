use std::rc::Rc;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    adapter::model::app::{
        signup_process::ParseIdError as SignupProcessParseIdError,
        user::ParseIdError as UserParseIdError,
    },
    application::gateway::repository::{
        signup_process::Record as SignupProcessRecord, user::Record as UserRecord,
    },
    domain::entity::{
        signup_process::{Completed, EmailAdded, Initialized, SignupState},
        user,
    },
};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum SignupProcessState {
    Initialized { username: String },
    EmailAdded { email: String },
    Completed,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SignupProcess {
    pub signup_process_id: String,
    pub chain: Vec<SignupProcessState>,
}

impl From<Rc<dyn SignupState>> for SignupProcessState {
    fn from(value: Rc<dyn SignupState>) -> Self {
        if let Some(Initialized { username }) = value.as_any().downcast_ref::<Initialized>() {
            SignupProcessState::Initialized {
                username: username.clone().to_string(),
            }
        } else if let Some(EmailAdded { email }) = value.as_any().downcast_ref::<EmailAdded>() {
            SignupProcessState::EmailAdded {
                email: email.clone().to_string(),
            }
        } else if let Some(Completed) = value.as_any().downcast_ref::<Completed>() {
            SignupProcessState::Completed
        } else {
            unreachable!();
        }
    }
}

impl From<SignupProcessState> for Rc<dyn SignupState> {
    fn from(value: SignupProcessState) -> Rc<dyn SignupState> {
        match value {
            SignupProcessState::Initialized { username } => Rc::new(Initialized {
                username: user::UserName::new(username),
            }),
            SignupProcessState::EmailAdded { email } => Rc::new(EmailAdded {
                email: user::Email::new(email),
            }),
            SignupProcessState::Completed => Rc::new(Completed),
        }
    }
}

impl TryInto<SignupProcessRecord> for SignupProcess {
    type Error = SignupProcessParseIdError;
    fn try_into(self) -> Result<SignupProcessRecord, Self::Error> {
        let id = self
            .signup_process_id
            .parse::<Uuid>()
            .map_err(|_| SignupProcessParseIdError)?;
        let mut chain: Vec<Rc<dyn SignupState>> = Vec::new();
        for state in self.chain {
            chain.push(state.into());
        }
        let state = chain.last().unwrap().clone();
        Ok(SignupProcessRecord {
            id: id.into(),
            chain,
            state,
        })
    }
}

impl From<SignupProcessRecord> for SignupProcess {
    fn from(value: SignupProcessRecord) -> SignupProcess {
        let mut chain: Vec<SignupProcessState> = Vec::new();
        for state in value.chain {
            chain.push(state.into());
        }
        SignupProcess {
            signup_process_id: value.id.to_string(),
            chain,
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
