use crate::domain::{
    entity::user::{Email, Password, UserName},
    value_object::{self},
};

#[derive(Debug, Clone)]
pub struct SignupProcessValue;

pub type Id = value_object::Id<SignupProcessValue>;

#[derive(Debug, Clone)]
pub enum SignupStateEnum {
    Initialized {
        email: Email,
    },
    EmailVerified {
        email: Email,
    },
    VerificationTimedOut {
        email: Email,
    },
    CompletionTimedOut {
        email: Email,
    },
    Completed {
        email: Email,
        username: UserName,
        password: Password,
    },
    ForDeletion,
}

pub trait SignupStateTrait: TryFrom<SignupStateEnum> + Into<SignupStateEnum> + Clone {}

pub trait Idable {
    fn id(&self) -> Id;
}
pub trait Delatable: Idable {
    fn delete(self) -> SignupProcess<ForDeletion>
    where
        Self: Sized,
    {
        SignupProcess {
            id: self.id(),
            state: ForDeletion {},
        }
    }
}

#[derive(Debug, Clone)]
pub struct Initialized {
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct EmailVerified {
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct VerificationTimedOut {
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct CompletionTimedOut {
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

impl SignupStateTrait for Initialized {}
impl SignupStateTrait for EmailVerified {}
impl SignupStateTrait for VerificationTimedOut {}
impl SignupStateTrait for CompletionTimedOut {}
impl SignupStateTrait for Completed {}
impl SignupStateTrait for ForDeletion {}

#[derive(Debug, Clone)]
pub struct SignupProcess<S: SignupStateTrait> {
    id: Id,
    state: S,
}

impl<S: SignupStateTrait> SignupProcess<S> {
    pub fn state(&self) -> &S {
        // chain is never empty
        &self.state
    }
}

impl<S: SignupStateTrait> Idable for SignupProcess<S> {
    fn id(&self) -> Id {
        self.id
    }
}

impl SignupProcess<Initialized> {
    pub fn new(id: Id, email: Email) -> Self {
        let state = Initialized { email };
        Self { id, state }
    }
    pub fn verify_email(self) -> SignupProcess<EmailVerified> {
        let state = EmailVerified {
            email: self.state.email,
        };
        SignupProcess { id: self.id, state }
    }
    pub fn verification_timed_out(self) -> SignupProcess<VerificationTimedOut> {
        let state = VerificationTimedOut {
            email: self.state.email,
        };
        SignupProcess { id: self.id, state }
    }
}

impl Delatable for SignupProcess<VerificationTimedOut> {}

impl SignupProcess<VerificationTimedOut> {
    pub fn extend_verification_time(self) -> SignupProcess<Initialized> {
        let state = Initialized {
            email: self.state.email,
        };
        SignupProcess { id: self.id, state }
    }
}

impl SignupProcess<EmailVerified> {
    pub fn complete(self, username: UserName, password: Password) -> SignupProcess<Completed> {
        let state = Completed {
            email: self.state.email,
            username,
            password,
        };
        SignupProcess { id: self.id, state }
    }
    pub fn completion_timed_out(self) -> SignupProcess<CompletionTimedOut> {
        let state = CompletionTimedOut {
            email: self.state.email,
        };
        SignupProcess { id: self.id, state }
    }
}

impl Delatable for SignupProcess<CompletionTimedOut> {}

impl SignupProcess<CompletionTimedOut> {
    pub fn extend_completion_time(self) -> SignupProcess<EmailVerified> {
        let state = EmailVerified {
            email: self.state.email,
        };
        SignupProcess { id: self.id, state }
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

impl TryFrom<SignupStateEnum> for Initialized {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::Initialized { email } => Ok(Self { email }),
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
impl TryFrom<SignupStateEnum> for VerificationTimedOut {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::VerificationTimedOut { email } => Ok(Self { email }),
            _ => Err(()),
        }
    }
}
impl TryFrom<SignupStateEnum> for CompletionTimedOut {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::CompletionTimedOut { email } => Ok(Self { email }),
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
impl<S: SignupStateTrait> TryFrom<(Id, SignupStateEnum)> for SignupProcess<S> {
    type Error = ();
    fn try_from(value: (Id, SignupStateEnum)) -> Result<Self, ()> {
        let (id, state) = value;
        match S::try_from(state.clone()) {
            Ok(state) => Ok(Self { id, state }),
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
impl Into<SignupStateEnum> for EmailVerified {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::EmailVerified { email: self.email }
    }
}
#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for VerificationTimedOut {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::VerificationTimedOut { email: self.email }
    }
}
#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for CompletionTimedOut {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::CompletionTimedOut { email: self.email }
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

#[cfg(test)]
mod tests {
    use super::*;

    mod signup_process {
        use super::*;
        use rstest::*;
        use uuid::Uuid;

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
            Email::new("test_email".to_string())
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
                SignupProcess::<Initialized>::try_from((id, initialized_state)).unwrap();
            if let SignupStateEnum::Initialized { email: state_email } = signup_process.state.into()
            {
                assert_eq!(state_email.to_string(), email.to_string());
            } else {
                unreachable!("Invalid state");
            }
        }
        #[rstest]
        // Test From wrong state enum
        fn test_try_from_wrong_state_enum(
            id: Id,
            email: Email,
            username: UserName,
            password: Password,
        ) {
            let initialized_state = SignupStateEnum::Initialized {
                email: email.clone(),
            };
            let email_verified_state = SignupStateEnum::EmailVerified {
                email: email.clone(),
            };
            let verification_timed_out_state = SignupStateEnum::VerificationTimedOut {
                email: email.clone(),
            };
            let completion_timed_out_state = SignupStateEnum::CompletionTimedOut {
                email: email.clone(),
            };
            let completed_state = SignupStateEnum::Completed {
                email: email.clone(),
                username: username.clone(),
                password: password.clone(),
            };
            let for_deletion_state = SignupStateEnum::ForDeletion;
            let res = SignupProcess::<Initialized>::try_from((id, initialized_state.clone()));
            assert!(res.is_ok());
            let res = SignupProcess::<EmailVerified>::try_from((id, email_verified_state.clone()));
            assert!(res.is_ok());
            let res = SignupProcess::<VerificationTimedOut>::try_from((
                id,
                verification_timed_out_state.clone(),
            ));
            assert!(res.is_ok());
            let res = SignupProcess::<CompletionTimedOut>::try_from((
                id,
                completion_timed_out_state.clone(),
            ));
            assert!(res.is_ok());
            let res = SignupProcess::<Completed>::try_from((id, completed_state.clone()));
            assert!(res.is_ok());
            let res = SignupProcess::<ForDeletion>::try_from((id, for_deletion_state.clone()));
            assert!(res.is_ok());

            let res = SignupProcess::<EmailVerified>::try_from((id, initialized_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<VerificationTimedOut>::try_from((id, initialized_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<CompletionTimedOut>::try_from((id, initialized_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<Completed>::try_from((id, initialized_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<ForDeletion>::try_from((id, initialized_state.clone()));
            assert!(res.is_err());

            let res = SignupProcess::<Initialized>::try_from((id, email_verified_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<VerificationTimedOut>::try_from((id, email_verified_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<CompletionTimedOut>::try_from((id, email_verified_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<Completed>::try_from((id, email_verified_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<ForDeletion>::try_from((id, email_verified_state.clone()));
            assert!(res.is_err());

            let res =
                SignupProcess::<Initialized>::try_from((id, verification_timed_out_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<EmailVerified>::try_from((
                id,
                verification_timed_out_state.clone(),
            ));
            assert!(res.is_err());
            let res = SignupProcess::<CompletionTimedOut>::try_from((
                id,
                verification_timed_out_state.clone(),
            ));
            assert!(res.is_err());
            let res =
                SignupProcess::<Completed>::try_from((id, verification_timed_out_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<ForDeletion>::try_from((id, verification_timed_out_state.clone()));
            assert!(res.is_err());

            let res =
                SignupProcess::<Initialized>::try_from((id, completion_timed_out_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<EmailVerified>::try_from((id, completion_timed_out_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<VerificationTimedOut>::try_from((
                id,
                completion_timed_out_state.clone(),
            ));
            assert!(res.is_err());
            let res =
                SignupProcess::<Completed>::try_from((id, completion_timed_out_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<ForDeletion>::try_from((id, completion_timed_out_state.clone()));
            assert!(res.is_err());

            let res = SignupProcess::<Initialized>::try_from((id, completed_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<EmailVerified>::try_from((id, completed_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<VerificationTimedOut>::try_from((id, completed_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<CompletionTimedOut>::try_from((id, completed_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<ForDeletion>::try_from((id, completed_state.clone()));
            assert!(res.is_err());

            let res = SignupProcess::<Initialized>::try_from((id, for_deletion_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<EmailVerified>::try_from((id, for_deletion_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<VerificationTimedOut>::try_from((id, for_deletion_state.clone()));
            assert!(res.is_err());
            let res =
                SignupProcess::<CompletionTimedOut>::try_from((id, for_deletion_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<Completed>::try_from((id, for_deletion_state.clone()));
            assert!(res.is_err());
        }
    }
}
