use std::{any::Any, rc::Rc};

use crate::domain::{
    entity::user::{Email, UserName},
    value_object,
};

#[derive(Debug, Clone)]
pub struct SignupProcessValue;

pub type Id = value_object::Id<SignupProcessValue>;

pub trait SignupState: Any {
    // ## Required for downcasting
    // implemented via impl_signup_state macro
    fn as_any(&self) -> &dyn Any;
}

macro_rules! impl_signup_state {
    ($($state:ident),+) => {
        $(
        impl SignupState for $state {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }
        )+
    };
}

#[derive(Debug, Clone)]
pub struct Initialized {
    pub username: UserName,
}
#[derive(Debug, Clone)]
pub struct EmailAdded {
    pub email: Email,
}
#[derive(Debug, Clone)]
pub struct Completed;

impl_signup_state!(Initialized, EmailAdded, Completed);

#[derive(Debug, Clone)]
pub struct SignupProcess<S: SignupState> {
    id: Id,
    chain: Vec<Rc<dyn SignupState>>,
    state: Rc<S>,
}

impl<S: SignupState> SignupProcess<S> {
    pub const fn id(&self) -> Id {
        self.id
    }
    pub const fn chain(&self) -> &Vec<Rc<dyn SignupState>> {
        &self.chain
    }
    fn transition<N: SignupState + 'static>(self, next: N) -> SignupProcess<N> {
        let mut chain = self.chain;
        let next = Rc::new(next);
        chain.push(next.clone());

        SignupProcess {
            id: self.id,
            chain,
            state: next,
        }
    }
    pub fn state(&self) -> Rc<dyn SignupState + 'static> {
        self.state.clone()
    }
}

// helper for reconstructing SignupProcess from dyn parameters.
impl<S: SignupState + Clone> TryFrom<(Id, Vec<Rc<dyn SignupState>>, Rc<dyn SignupState>)>
    for SignupProcess<S>
{
    type Error = ();
    fn try_from(
        value: (Id, Vec<Rc<dyn SignupState>>, Rc<dyn SignupState>),
    ) -> Result<Self, Self::Error> {
        if let Some(state) = value.2.as_any().downcast_ref::<S>() {
            Ok(Self {
                id: value.0,
                chain: value.1,
                state: Rc::new(state.clone()),
            })
        } else {
            Err(())
        }
    }
}

impl SignupProcess<Initialized> {
    pub fn new(id: Id, username: UserName) -> Self {
        let state = Rc::new(Initialized { username });
        Self {
            id,
            chain: vec![state.clone()],
            state,
        }
    }

    pub fn add_email(self, email: Email) -> SignupProcess<EmailAdded> {
        self.transition(EmailAdded { email })
    }
}

impl SignupProcess<EmailAdded> {
    pub fn complete(self) -> SignupProcess<Completed> {
        self.transition(Completed)
    }
}

impl SignupProcess<Completed> {
    // completed contians at least one Initialized state in it's chain thus the unreachable.
    pub fn username(&self) -> UserName {
        for item in &self.chain {
            if let Some(Initialized { username }) = item.as_any().downcast_ref::<Initialized>() {
                return username.clone();
            }
        }
        unreachable!();
    }
    // completed contians at least one EmailAdded state in it's chain thus the unreachable.
    pub fn email(&self) -> Email {
        for item in &self.chain {
            if let Some(EmailAdded { email }) = item.as_any().downcast_ref::<EmailAdded>() {
                return email.clone();
            }
        }
        unreachable!();
    }
}

impl std::fmt::Debug for dyn SignupState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(initialized) = self.as_any().downcast_ref::<Initialized>() {
            write!(f, "Initialized: {:?}", initialized)
        } else if let Some(email_added) = self.as_any().downcast_ref::<EmailAdded>() {
            write!(f, "EmailAdded: {:?}", email_added)
        } else if let Some(completed) = self.as_any().downcast_ref::<Completed>() {
            write!(f, "Completed: {:?}", completed)
        } else {
            unreachable!();
        }
    }
}
