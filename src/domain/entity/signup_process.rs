use crate::domain::{
    entity::user::{Email, UserName},
    value_object,
};

#[derive(Debug, Clone)]
pub struct SignupProcessValue;

pub type Id = value_object::Id<SignupProcessValue>;

#[derive(Debug, Clone)]
pub enum SignupStateEnum {
    Initialized { username: UserName },
    EmailAdded { username: UserName, email: Email },
    Completed { username: UserName, email: Email },
}

pub trait SignupStateTrait: TryFrom<SignupStateEnum> + Into<SignupStateEnum> + Clone {}

#[derive(Debug, Clone)]
pub struct Initialized {
    pub username: UserName,
}
#[derive(Debug, Clone)]
pub struct EmailAdded {
    pub username: UserName,
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct Completed {
    pub username: UserName,
    pub email: Email,
}

impl SignupStateTrait for Initialized {}
impl SignupStateTrait for EmailAdded {}
impl SignupStateTrait for Completed {}

#[derive(Debug, Clone)]
pub struct SignupProcess<S: SignupStateTrait> {
    id: Id,
    state: S,
}

impl<S: SignupStateTrait> SignupProcess<S> {
    pub const fn id(&self) -> Id {
        self.id
    }
    pub fn state(&self) -> &S {
        // chain is never empty
        &self.state
    }
}

impl SignupProcess<Initialized> {
    pub fn new(id: Id, username: UserName) -> Self {
        let state = Initialized { username };
        Self { id, state }
    }
    pub fn add_email(self, email: Email) -> SignupProcess<EmailAdded> {
        let state = EmailAdded {
            username: self.state.username,
            email,
        };
        SignupProcess { id: self.id, state }
    }
}

impl SignupProcess<EmailAdded> {
    pub fn complete(self) -> SignupProcess<Completed> {
        let state = Completed {
            username: self.state.username,
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
}

impl TryFrom<SignupStateEnum> for Initialized {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::Initialized { username } => Ok(Self { username }),
            _ => Err(()),
        }
    }
}
impl TryFrom<SignupStateEnum> for EmailAdded {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::EmailAdded { username, email } => Ok(Self { username, email }),
            _ => Err(()),
        }
    }
}
impl TryFrom<SignupStateEnum> for Completed {
    type Error = ();
    fn try_from(value: SignupStateEnum) -> Result<Self, Self::Error> {
        match value {
            SignupStateEnum::Completed { username, email } => Ok(Self { username, email }),
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
        SignupStateEnum::Initialized {
            username: self.username,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for EmailAdded {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::EmailAdded {
            username: self.username,
            email: self.email,
        }
    }
}
#[allow(clippy::from_over_into)]
impl Into<SignupStateEnum> for Completed {
    fn into(self) -> SignupStateEnum {
        SignupStateEnum::Completed {
            username: self.username,
            email: self.email,
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
            UserName::new("test".to_string())
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
        fn test_signup_process_initialization(id: Id, username: UserName) {
            let signup_process = SignupProcess::new(id, username.clone());
            assert_eq!(signup_process.id, id);
            assert_eq!(
                signup_process.state.username.to_string(),
                username.to_string()
            );
        }
        #[rstest]
        // Test that the state method returns the correct state for a SignupProcess<Initialized>
        fn test_signup_process_state(id: Id, username: UserName) {
            let initialized_state = SignupStateEnum::Initialized {
                username: username.clone(),
            };
            let signup_process =
                SignupProcess::<Initialized>::try_from((id, initialized_state)).unwrap();
            if let SignupStateEnum::Initialized {
                username: state_username,
            } = signup_process.state.into()
            {
                assert_eq!(state_username.to_string(), username.to_string());
            } else {
                unreachable!("Invalid state");
            }
        }
        #[rstest]
        // Test that adding an email transitions the state from Initialized to EmailAdded
        fn test_signup_process_add_email(id: Id, username: UserName, email: Email) {
            let initialized_state = SignupStateEnum::Initialized {
                username: username.clone(),
            };
            let signup_process =
                SignupProcess::<Initialized>::try_from((id, initialized_state)).unwrap();
            let signup_process = signup_process.add_email(email.clone());
            if let SignupStateEnum::EmailAdded {
                username: state_username,
                email: state_email,
            } = signup_process.state.into()
            {
                assert_eq!(state_username.to_string(), username.to_string());
                assert_eq!(state_email.to_string(), email.to_string());
            } else {
                unreachable!("Invalid state");
            }
        }
        #[rstest]
        // Test From wrong state enum
        fn test_try_from_wrong_state_enum(id: Id, username: UserName, email: Email) {
            let initialized_state = SignupStateEnum::Initialized {
                username: username.clone(),
            };
            let email_added_state = SignupStateEnum::EmailAdded {
                username: username.clone(),
                email: email.clone(),
            };
            let completed_state = SignupStateEnum::Completed {
                username: username.clone(),
                email: email.clone(),
            };
            let res = SignupProcess::<Initialized>::try_from((id, initialized_state.clone()));
            assert!(res.is_ok());
            let res = SignupProcess::<EmailAdded>::try_from((id, email_added_state.clone()));
            assert!(res.is_ok());
            let res = SignupProcess::<Completed>::try_from((id, completed_state.clone()));
            assert!(res.is_ok());
            let res = SignupProcess::<EmailAdded>::try_from((id, initialized_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<Completed>::try_from((id, initialized_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<Initialized>::try_from((id, email_added_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<Completed>::try_from((id, email_added_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<Initialized>::try_from((id, completed_state.clone()));
            assert!(res.is_err());
            let res = SignupProcess::<EmailAdded>::try_from((id, completed_state.clone()));
            assert!(res.is_err());
        }
        #[rstest]
        // Test that the SignupProcess<EmailAdded> transitions to Completed correctly
        fn test_signup_process_transition_to_completed(id: Id, username: UserName, email: Email) {
            let email_added_state = SignupStateEnum::EmailAdded {
                username: username.clone(),
                email: email.clone(),
            };
            let signup_process =
                SignupProcess::<EmailAdded>::try_from((id, email_added_state)).unwrap();
            let signup_process = signup_process.complete();
            if let SignupStateEnum::Completed {
                username: _,
                email: _,
            } = signup_process.state.into()
            {
            } else {
                unreachable!("Invalid state");
            }
        }
        #[rstest]
        // Test that the username method returns the correct username adn email for a SignupProcess<Completed>
        fn test_signup_process_completed_username_and_email(
            id: Id,
            username: UserName,
            email: Email,
        ) {
            let completed_state = SignupStateEnum::Completed {
                username: username.clone(),
                email: email.clone(),
            };
            let signup_process =
                SignupProcess::<Completed>::try_from((id, completed_state)).unwrap();
            if let SignupStateEnum::Completed {
                username: _,
                email: _,
            } = signup_process.state.clone().into()
            {
                assert_eq!(
                    signup_process.state.username.to_string(),
                    username.to_string()
                );
                assert_eq!(signup_process.state.email.to_string(), email.to_string());
            } else {
                unreachable!("Invalid state");
            }
        }
    }
}
