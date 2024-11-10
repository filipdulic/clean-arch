use std::sync::Arc;

use crate::{
    entity::user::{Email, Password, UserName},
    value_object::{self},
};

use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct SignupProcessValue;

pub type Id = value_object::Id<SignupProcessValue>;

#[derive(Debug, Clone)]
pub enum SignupStateEnum {
    Initialized {
        email: Email,
    },
    VerificationEmailSent {
        email: Email,
    },
    EmailVerified {
        email: Email,
    },
    Completed {
        email: Email,
        username: UserName,
        password: Password,
    },
    ForDeletion,
    Failed {
        previous_state: Arc<SignupStateEnum>,
        error: Error,
    },
}

pub trait SignupStateTrait: TryFrom<SignupStateEnum> + Into<SignupStateEnum> + Clone {}
#[derive(Debug, Clone)]
pub struct Initialized {
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct VerificationEmailSent {
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct EmailVerified {
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct Completed {
    pub email: Email,
    pub username: UserName,
    pub password: Password,
}
#[derive(Debug, Clone)]
pub struct ForDeletion {}

#[derive(Debug, Clone)]
pub enum Error {
    TokenGenrationFailed,
    VerificationEmailSendError,
    VerificationTimedOut,
    CompletionTimedOut,
}

#[derive(Debug, Clone)]
pub struct Failed<S: SignupStateTrait> {
    pub previous_state: S,
    pub error: Error,
}

impl SignupStateTrait for Initialized {}
impl SignupStateTrait for VerificationEmailSent {}
impl SignupStateTrait for EmailVerified {}
impl SignupStateTrait for Completed {}
impl SignupStateTrait for ForDeletion {}
impl<S: SignupStateTrait> SignupStateTrait for Failed<S> {}

#[derive(Debug, Clone)]
pub struct SignupProcess<S: SignupStateTrait> {
    id: Id,
    state: S,
    entered_at: DateTime<Utc>,
}

impl<S: SignupStateTrait> SignupProcess<S> {
    pub fn entered_at(&self) -> DateTime<Utc> {
        self.entered_at
    }
    pub fn state(&self) -> &S {
        &self.state
    }
    pub fn id(&self) -> Id {
        self.id
    }
    pub fn fail(&self, error: Error) -> SignupProcess<Failed<S>> {
        let state = Failed {
            previous_state: self.state.clone(),
            error,
        };
        SignupProcess {
            id: self.id,
            state,
            entered_at: Utc::now(),
        }
    }
}

impl SignupProcess<Initialized> {
    pub fn new(id: Id, email: Email) -> Self {
        let state = Initialized { email };
        Self {
            id,
            state,
            entered_at: Utc::now(),
        }
    }
    pub fn send_verification_email(self) -> SignupProcess<VerificationEmailSent> {
        let state = VerificationEmailSent {
            email: self.state.email,
        };
        SignupProcess {
            id: self.id,
            state,
            entered_at: Utc::now(),
        }
    }
}

impl SignupProcess<VerificationEmailSent> {
    pub fn verify_email(self) -> SignupProcess<EmailVerified> {
        let state = EmailVerified {
            email: self.state.email,
        };
        SignupProcess {
            id: self.id,
            state,
            entered_at: Utc::now(),
        }
    }
}

impl SignupProcess<EmailVerified> {
    pub fn complete(self, username: UserName, password: Password) -> SignupProcess<Completed> {
        let state = Completed {
            email: self.state.email,
            username,
            password,
        };
        SignupProcess {
            id: self.id,
            state,
            entered_at: Utc::now(),
        }
    }
}

impl SignupProcess<Completed> {
    pub fn username(&self) -> UserName {
        self.state.username.clone()
    }
    pub fn email(&self) -> Email {
        self.state.email.clone()
    }
    pub fn password(&self) -> Password {
        self.state.password.clone()
    }
}

impl<S: SignupStateTrait> SignupProcess<Failed<S>> {
    pub fn recover(&self) -> SignupProcess<S> {
        let state = self.state.previous_state.clone();
        SignupProcess {
            id: self.id,
            state,
            entered_at: Utc::now(),
        }
    }
    pub fn delete(self) -> SignupProcess<ForDeletion> {
        let state = ForDeletion {};
        SignupProcess {
            id: self.id,
            state,
            entered_at: Utc::now(),
        }
    }
}

impl TryFrom<SignupStateEnum> for Initialized {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::Initialized { email } => Ok(Self { email }),
            _ => Err(()),
        }
    }
}
impl TryFrom<SignupStateEnum> for VerificationEmailSent {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::VerificationEmailSent { email } => Ok(Self { email }),
            _ => Err(()),
        }
    }
}
impl TryFrom<SignupStateEnum> for EmailVerified {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::EmailVerified { email } => Ok(Self { email }),
            _ => Err(()),
        }
    }
}

impl TryFrom<SignupStateEnum> for Completed {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::Completed {
                username,
                email,
                password,
            } => Ok(Self {
                email,
                username,
                password,
            }),
            _ => Err(()),
        }
    }
}
impl TryFrom<SignupStateEnum> for ForDeletion {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::ForDeletion => Ok(Self {}),
            _ => Err(()),
        }
    }
}

impl<S: SignupStateTrait> TryFrom<SignupStateEnum> for Failed<S> {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::Failed {
                previous_state,
                error,
            } => Ok(Self {
                previous_state: S::try_from(previous_state.as_ref().clone()).map_err(|_| ())?,
                error,
            }),
            _ => Err(()),
        }
    }
}
impl<S: SignupStateTrait> TryFrom<(Id, SignupStateEnum, DateTime<Utc>)> for SignupProcess<S> {
    type Error = ();
    fn try_from(value: (Id, SignupStateEnum, DateTime<Utc>)) -> Result<Self, ()> {
        let (id, state, entered_at) = value;
        match S::try_from(state.clone()) {
            Ok(state) => Ok(Self {
                id,
                state,
                entered_at,
            }),
            Err(_) => Err(()),
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for Initialized {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::Initialized { email: self.email }
    }
}
#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for VerificationEmailSent {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::VerificationEmailSent { email: self.email }
    }
}
#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for EmailVerified {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::EmailVerified { email: self.email }
    }
}
#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for ForDeletion {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::ForDeletion
    }
}

#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for Completed {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::Completed {
            email: self.email,
            username: self.username,
            password: self.password,
        }
    }
}

#[allow(clippy::from_over_into)]
impl<S: SignupStateTrait> Into<SignupStateEnum> for Failed<S> {
    fn into(self) -> SignupStateEnum {
        let previous_state: SignupStateEnum = self.previous_state.into();
        SignupStateEnum::Failed {
            previous_state: Arc::new(previous_state),
            error: self.error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    mod signup_process {
        use super::*;
        use rstest::*;

        #[fixture]
        pub fn username() -> UserName {
            UserName::new("test_username".to_string())
        }
        #[fixture]
        pub fn password() -> Password {
            Password::new("test_pass".to_string())
        }
        #[fixture]
        pub fn id() -> Id {
            Id::new(Uuid::new_v4())
        }
        #[fixture]
        pub fn email() -> Email {
            Email::new("test_email")
        }
        #[rstest]
        // Test that a new SignupProcess<Initialized> is created with the correct id and username
        fn test_signup_process_initialization(id: Id, email: Email) {
            let signup_process = SignupProcess::new(id, email.clone());
            assert_eq!(signup_process.id, id);
            assert_eq!(signup_process.state.email.to_string(), email.to_string());
        }
        #[rstest]
        // Test that the state method returns the correct state for a SignupProcess<Initialized>
        fn test_signup_process_state(id: Id, email: Email) {
            let initialized_state = SignupStateEnum::Initialized {
                email: email.clone(),
            };
            let signup_process =
                SignupProcess::<Initialized>::try_from((id, initialized_state, Utc::now()))
                    .unwrap();
            if let SignupStateEnum::Initialized { email: state_email } = signup_process.state.into()
            {
                assert_eq!(state_email.to_string(), email.to_string());
            } else {
                unreachable!("Invalid state");
            }
        }
    }
}
