use crate::domain::value_object;

use super::signup_process::{Completed, SignupProcess};

pub type Id = value_object::Id<User>;
pub type UserName = value_object::UserName<User>;
pub type Email = value_object::Email<User>;
pub type Password = value_object::Password<User>;

#[derive(Debug, Clone)]
pub struct User {
    id: Id,
    email: Email,
    username: UserName,
    password: Password,
}

impl User {
    pub fn new(id: Id, email: Email, username: UserName, password: Password) -> Self {
        // Never construct an area of life with invalid name
        debug_assert!(username.as_ref().len() <= UserName::max_len());
        debug_assert!(username.as_ref().len() >= UserName::min_len());

        debug_assert!(email.as_ref().len() <= Email::max_len());
        debug_assert!(email.as_ref().len() >= Email::min_len());

        debug_assert!(password.as_ref().len() <= Password::max_len());
        debug_assert!(password.as_ref().len() >= Password::min_len());

        Self {
            id,
            email,
            username,
            password,
        }
    }
    pub const fn id(&self) -> Id {
        self.id
    }
    pub const fn email(&self) -> &Email {
        &self.email
    }
    pub const fn username(&self) -> &UserName {
        &self.username
    }
    pub const fn password(&self) -> &Password {
        &self.password
    }
}

impl From<SignupProcess<Completed>> for User {
    fn from(signup_process: SignupProcess<Completed>) -> Self {
        Self {
            id: Id::new(signup_process.id()),
            email: signup_process.email(),
            username: signup_process.username(),
            password: signup_process.password(),
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

const MAX_PASSWORD_LEN: usize = 30;
const MIN_PASSWORD_LEN: usize = 5;

impl Password {
    pub const fn min_len() -> usize {
        MIN_PASSWORD_LEN
    }
    pub const fn max_len() -> usize {
        MAX_PASSWORD_LEN
    }
}
