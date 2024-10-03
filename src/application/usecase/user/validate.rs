use crate::domain::entity::user::{Email, UserName};
use thiserror::Error;

#[derive(Debug)]
pub struct Request<'a> {
    pub username: &'a str,
    pub email: &'a str,
}
pub type Response = Result<(), UserInvalidity>;

#[derive(Debug, Error)]
pub enum UserInvalidity {
    #[error(transparent)]
    UserName(#[from] UserNameInvalidity),
    #[error(transparent)]
    Email(#[from] EmailInvalidity),
}

#[derive(Debug, Error)]
pub enum UserNameInvalidity {
    #[error("The user name must have at least {min} but has {actual} chars")]
    MinLength { min: usize, actual: usize },
    #[error("The user name must have at most {max} but has {actual} chars")]
    MaxLength { max: usize, actual: usize },
}

#[derive(Debug, Error)]
pub enum EmailInvalidity {
    #[error("The email must have at least {min} but has {actual} chars")]
    MinLength { min: usize, actual: usize },
    #[error("The email must have at most {max} but has {actual} chars")]
    MaxLength { max: usize, actual: usize },
}

pub fn validate_user_properties(req: &Request) -> Response {
    log::debug!("Validate area of life properties {:?}", req);
    validate_username(req.username).map_err(UserInvalidity::UserName)?;
    validate_email(req.email).map_err(UserInvalidity::Email)?;
    Ok(())
}

fn validate_username(username: &str) -> Result<(), UserNameInvalidity> {
    let actual = username.len();
    let min = UserName::min_len();

    if actual < min {
        return Err(UserNameInvalidity::MinLength { min, actual });
    }
    let max = UserName::max_len();
    if actual > max {
        return Err(UserNameInvalidity::MaxLength { max, actual });
    }
    Ok(())
}

fn validate_email(email: &str) -> Result<(), EmailInvalidity> {
    let actual = email.len();
    let min = Email::min_len();

    if actual < min {
        return Err(EmailInvalidity::MinLength { min, actual });
    }
    let max = Email::max_len();
    if actual > max {
        return Err(EmailInvalidity::MaxLength { max, actual });
    }
    Ok(())
}
