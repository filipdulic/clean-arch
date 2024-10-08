use crate::domain::value_object;

use super::signup_process::{Completed, SignupProcess};

pub type Id = value_object::Id<User>;
pub type UserName = value_object::UserName<User>;
pub type Email = value_object::Email<User>;

#[derive(Debug, Clone)]
pub struct User {
    id: Id,
    username: UserName,
    email: Email,
}

impl User {
    pub fn new(id: Id, username: UserName, email: Email) -> Self {
        // Never construct an area of life with invalid name
        debug_assert!(username.as_ref().len() <= UserName::max_len());
        debug_assert!(username.as_ref().len() >= UserName::min_len());

        debug_assert!(email.as_ref().len() <= Email::max_len());
        debug_assert!(email.as_ref().len() >= Email::min_len());
        Self {
            id,
            username,
            email,
        }
    }
    pub const fn id(&self) -> Id {
        self.id
    }
    pub const fn username(&self) -> &UserName {
        &self.username
    }
    pub const fn email(&self) -> &Email {
        &self.email
    }
}

impl From<SignupProcess<Completed>> for User {
    fn from(signup_process: SignupProcess<Completed>) -> Self {
        Self {
            id: Id::new(signup_process.id()),
            username: signup_process.username(),
            email: signup_process.email(),
        }
    }
}

const MAX_NAME_LEN: usize = 30;
const MIN_NAME_LEN: usize = 5;

impl UserName {
    pub const fn min_len() -> usize {
        MIN_NAME_LEN
    }
    pub const fn max_len() -> usize {
        MAX_NAME_LEN
    }
}

const MAX_EMAIL_LEN: usize = 30;
const MIN_EMAIL_LEN: usize = 5;

impl Email {
    pub const fn min_len() -> usize {
        MIN_EMAIL_LEN
    }
    pub const fn max_len() -> usize {
        MAX_EMAIL_LEN
    }
}
