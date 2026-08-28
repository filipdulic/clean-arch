use std::{str::FromStr, sync::Arc};

use chrono::{DateTime, Utc};
use sqlx::prelude::FromRow;

use ca_application::gateway::database::signup_process::Record;
use ca_domain::{
    entity::{
        signup_process::{Error as SignupError, Id, SignupStateEnum},
        user::{Email, Password},
    },
    value_object::UserName,
};

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

fn from_process_and_previous(
    (value, prev_state): (&SignupProcessState, &Option<SignupStateEnum>),
) -> Result<SignupStateEnum, ca_application::gateway::database::signup_process::GetError> {
    use ca_application::gateway::database::signup_process::GetError;

    match value.state.as_str() {
        "Initialized" => Ok(SignupStateEnum::Initialized {
            email: Email::new(value.email.as_ref().ok_or(GetError::IncorrectState)?),
        }),
        "VerificationEmailSent" => Ok(SignupStateEnum::VerificationEmailSent {
            email: Email::new(value.email.as_ref().ok_or(GetError::IncorrectState)?),
        }),
        "EmailVerified" => Ok(SignupStateEnum::EmailVerified {
            email: Email::new(value.email.as_ref().ok_or(GetError::IncorrectState)?),
        }),
        "Completed" => Ok(SignupStateEnum::Completed {
            email: Email::new(value.email.as_ref().ok_or(GetError::IncorrectState)?),
            username: UserName::new(value.username.as_ref().ok_or(GetError::IncorrectState)?),
            password: Password::new(value.password.as_ref().ok_or(GetError::IncorrectState)?),
        }),
        "ForDeletion" => Ok(SignupStateEnum::ForDeletion),
        "Failed" => Ok(SignupStateEnum::Failed {
            previous_state: Arc::new(prev_state.clone().ok_or(GetError::IncorrectState)?),
            error: value
                .error
                .as_ref()
                .ok_or(GetError::IncorrectState)?
                .parse::<SignupError>()
                .map_err(|_| GetError::IncorrectState)?,
        }),
        _ => Err(GetError::IncorrectState),
    }
}

pub fn from_chain(
    chain: Vec<SignupProcessState>,
) -> Result<Vec<Record>, ca_application::gateway::database::signup_process::GetError> {
    use ca_application::gateway::database::signup_process::GetError;

    let mut previous: Option<SignupStateEnum> = None;
    let mut records = Vec::with_capacity(chain.len());
    for process in chain {
        let state = from_process_and_previous((&process, &previous))?;
        let id = uuid::Uuid::from_str(&process.signup_id).map_err(|_| GetError::IncorrectState)?;
        previous = Some(state.clone());
        records.push(Record {
            id: Id::from(id),
            state,
            entered_at: process.entered_at,
        });
    }
    Ok(records)
}
