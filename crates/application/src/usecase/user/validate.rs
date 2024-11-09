use ca_domain::entity::user::{Email, Password, UserName};
use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
pub struct Request<'a> {
    pub username: &'a str,
    pub email: &'a str,
    pub password: &'a str,
}
pub type Response = Result<(), UserInvalidity>;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Error)]
pub enum UserInvalidity {
    #[error(transparent)]
    UserName(#[from] UserNameInvalidity),
    #[error(transparent)]
    Email(#[from] EmailInvalidity),
    #[error(transparent)]
    Password(#[from] PasswordInvalidity),
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Error)]
pub enum UserNameInvalidity {
    #[error("The user name must have at least {min} but has {actual} chars")]
    MinLength { min: usize, actual: usize },
    #[error("The user name must have at most {max} but has {actual} chars")]
    MaxLength { max: usize, actual: usize },
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Error)]
pub enum EmailInvalidity {
    #[error("The email must have at least {min} but has {actual} chars")]
    MinLength { min: usize, actual: usize },
    #[error("The email must have at most {max} but has {actual} chars")]
    MaxLength { max: usize, actual: usize },
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Error)]
pub enum PasswordInvalidity {
    #[error("The password must have at least {min} but has {actual} chars")]
    MinLength { min: usize, actual: usize },
    #[error("The password must have at most {max} but has {actual} chars")]
    MaxLength { max: usize, actual: usize },
}

pub fn validate_user_properties(req: &Request) -> Response {
    log::debug!("Validate area of life properties {:?}", req);
    validate_username(req.username).map_err(UserInvalidity::UserName)?;
    validate_email(req.email).map_err(UserInvalidity::Email)?;
    validate_password(req.password).map_err(UserInvalidity::Password)?;
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

fn validate_password(password: &str) -> Result<(), PasswordInvalidity> {
    let actual = password.len();
    let min = Password::min_len();

    if actual < min {
        return Err(PasswordInvalidity::MinLength { min, actual });
    }
    let max = Password::max_len();
    if actual > max {
        return Err(PasswordInvalidity::MaxLength { max, actual });
    }
    Ok(())
}
