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
pub trait SignupStateTrait {}

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
    state: SignupStateEnum,
    _phantom: std::marker::PhantomData<S>,
}

impl<S: SignupStateTrait> SignupProcess<S> {
    pub const fn id(&self) -> Id {
        self.id
    }
    pub fn state(&self) -> &SignupStateEnum {
        // chain is never empty
        &self.state
    }
}

impl SignupProcess<Initialized> {
    pub fn new(id: Id, username: UserName) -> Self {
        let state = SignupStateEnum::Initialized { username };
        Self {
            id,
            state,
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn username(&self) -> UserName {
        if let SignupStateEnum::Initialized { username } = &self.state {
            username.clone()
        } else {
            unreachable!()
        }
    }

    pub fn add_email(self, email: Email) -> SignupProcess<EmailAdded> {
        let state = SignupStateEnum::EmailAdded {
            username: self.username(),
            email,
        };
        SignupProcess {
            id: self.id,
            state,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl SignupProcess<EmailAdded> {
    pub fn username(&self) -> UserName {
        if let SignupStateEnum::EmailAdded { username, .. } = &self.state {
            username.clone()
        } else {
            unreachable!()
        }
    }
    pub fn email(&self) -> Email {
        if let SignupStateEnum::EmailAdded { email, .. } = &self.state {
            email.clone()
        } else {
            unreachable!()
        }
    }
    pub fn complete(self) -> SignupProcess<Completed> {
        let state = SignupStateEnum::Completed {
            username: self.username(),
            email: self.email(),
        };
        SignupProcess {
            id: self.id,
            state,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl SignupProcess<Completed> {
    pub fn username(&self) -> UserName {
        if let SignupStateEnum::Completed { username, .. } = &self.state {
            username.clone()
        } else {
            unreachable!()
        }
    }
    pub fn email(&self) -> Email {
        if let SignupStateEnum::Completed { email, .. } = &self.state {
            email.clone()
        } else {
            unreachable!()
        }
    }
}

// helper for reconstructing SignupProcess from dyn parameters.
impl<S: SignupStateTrait> From<(Id, SignupStateEnum)> for SignupProcess<S> {
    fn from(value: (Id, SignupStateEnum)) -> Self {
        let (id, state) = value;
        Self {
            id,
            state,
            _phantom: std::marker::PhantomData,
        }
    }
}
