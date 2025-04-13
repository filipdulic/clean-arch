use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use ca_application::gateway::repository::signup_process::Record;
use ca_domain::{
    entity::{
        signup_process::{Error as SignupError, Id, SignupStateEnum},
        user::{Email, Password},
    },
    value_object::UserName,
};

// NOTE:

#[derive(Debug, Clone, FromRow)]
pub struct SignupProcessState {
    #[sqlx(rename = "id")]
    pub signup_id: String, // non null not unique
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub entered_at: DateTime<Utc>,
    pub state: String,
    pub error: Option<String>,
}

impl From<Record> for SignupProcessState {
    fn from(record: Record) -> Self {
        match record.state {
            SignupStateEnum::Initialized { email } => SignupProcessState {
                signup_id: record.id.to_string(),
                username: None,
                email: Some(email.to_string()),
                password: None,
                entered_at: record.entered_at,
                state: "Initialized".to_string(),
                error: None,
            },
            SignupStateEnum::VerificationEmailSent { email } => SignupProcessState {
                signup_id: record.id.to_string(),
                username: None,
                email: Some(email.to_string()),
                password: None,
                entered_at: record.entered_at,
                state: "VerificationEmailSent".to_string(),
                error: None,
            },
            SignupStateEnum::EmailVerified { email } => SignupProcessState {
                signup_id: record.id.to_string(),
                username: None,
                email: Some(email.to_string()),
                password: None,
                entered_at: record.entered_at,
                state: "EmailVerified".to_string(),
                error: None,
            },
            SignupStateEnum::Completed {
                email,
                username,
                password,
            } => SignupProcessState {
                signup_id: record.id.to_string(),
                username: Some(username.to_string()),
                email: Some(email.to_string()),
                password: Some(password.to_string()),
                entered_at: record.entered_at,
                state: "Completed".to_string(),
                error: None,
            },
            SignupStateEnum::ForDeletion => SignupProcessState {
                signup_id: record.id.to_string(),
                username: None,
                email: None,
                password: None,
                entered_at: record.entered_at,
                state: "ForDeletion".to_string(),
                error: None,
            },
            SignupStateEnum::Failed {
                #[allow(unused_variables)]
                previous_state,
                error,
            } => SignupProcessState {
                signup_id: record.id.to_string(),
                username: None,
                email: None,
                password: None,
                entered_at: record.entered_at,
                state: "Failed".to_string(),
                error: Some(error.to_string()),
            },
        }
    }
}

fn from_proces_and_prev(
    (value, prev_state): (&SignupProcessState, &Option<SignupStateEnum>),
) -> SignupStateEnum {
    match value.state.as_str() {
        "Initialized" => SignupStateEnum::Initialized {
            email: Email::new(value.email.as_ref().unwrap()),
        },
        "VerificationEmailSent" => SignupStateEnum::VerificationEmailSent {
            email: Email::new(value.email.as_ref().unwrap()),
        },
        "EmailVerified" => SignupStateEnum::EmailVerified {
            email: Email::new(value.email.as_ref().unwrap()),
        },
        "Completed" => SignupStateEnum::Completed {
            email: Email::new(value.email.as_ref().unwrap()),
            username: UserName::new(value.username.as_ref().unwrap()),
            password: Password::new(value.password.as_ref().unwrap()),
        },
        "ForDeletion" => SignupStateEnum::ForDeletion,
        "Failed" => SignupStateEnum::Failed {
            // temp previous state
            previous_state: Arc::new(prev_state.clone().unwrap()),
            error: SignupError::from_str(value.error.as_ref().unwrap()).unwrap(),
        },
        _ => panic!("Invalid state"),
    }
}

pub fn from_chain(chain: Vec<SignupProcessState>) -> Vec<Record> {
    let mut previous: Option<SignupStateEnum> = None;
    chain
        .into_iter()
        .map(|process| {
            let state = from_proces_and_prev((&process, &previous));
            previous = Some(state.clone());
            Record {
                id: Id::from(uuid::Uuid::from_str(&process.signup_id).unwrap()),
                state: state.clone(),
                entered_at: process.entered_at,
            }
        })
        .collect::<Vec<_>>()
}
