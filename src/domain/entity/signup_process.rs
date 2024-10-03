use std::{any::Any, rc::Rc};

use crate::domain::{
    entity::user::{Email, UserName},
    value_object,
};

#[derive(Debug, Clone)]
pub struct SignupProcessValue;

pub type Id = value_object::Id<SignupProcessValue>;

pub trait SignupState: Any {
    fn as_any(&self) -> &dyn Any;
}
#[derive(Clone)]
pub struct SignupProcess<S: SignupState> {
    pub id: Id,
    pub chain: Vec<Rc<dyn SignupState>>,
    pub state: Rc<S>,
}

impl<S: SignupState> SignupProcess<S> {
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
    pub fn state(&self) -> &S {
        self.state.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct Initialized {
    username: UserName,
}
#[derive(Debug, Clone)]
pub struct EmailAdded {
    email: Email,
}
#[derive(Debug, Clone)]
pub struct Completed;

impl SignupState for Initialized {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl SignupState for EmailAdded {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
impl SignupState for Completed {
    fn as_any(&self) -> &dyn Any {
        self
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
    fn try_username(&self) -> Option<UserName> {
        for item in &self.chain {
            if let Some(Initialized { username }) = item.as_any().downcast_ref::<Initialized>() {
                return Some(username.clone());
            }
        }
        None
    }

    pub fn username(&self) -> UserName {
        self.try_username().unwrap()
    }

    fn try_email(&self) -> Option<Email> {
        for item in &self.chain {
            if let Some(EmailAdded { email }) = item.as_any().downcast_ref::<EmailAdded>() {
                return Some(email.clone());
            }
        }
        None
    }

    pub fn email(&self) -> Email {
        self.try_email().unwrap()
    }
}
