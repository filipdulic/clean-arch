use std::future::Future;

use serde::Serialize;
use thiserror::Error;
pub struct EmailAddress(String);
impl EmailAddress {
    pub fn new(address: &str) -> Self {
        Self(address.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Debug, Error, Serialize, PartialEq)]
pub enum EmailServiceError {
    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(String),
    #[error("Failed to send email")]
    SendEmailFailed,
}
pub trait EmailService {
    fn send_email(
        &self,
        to: EmailAddress,
        subject: &str,
        body: &str,
    ) -> impl Future<Output = Result<(), EmailServiceError>>;
}

pub trait EmailVerificationService {
    fn send_verification_email(
        &self,
        to: EmailAddress,
        token: &str,
    ) -> impl Future<Output = Result<(), EmailServiceError>>;
}

#[cfg(test)]
pub mod mock {
    use mockall::mock;

    mock! {
        pub EmailVerificationService {}
        impl super::EmailVerificationService for EmailVerificationService {
            async fn send_verification_email(
                &self,
                to: super::EmailAddress,
                token: &str,
            ) -> Result<(), super::EmailServiceError>;
        }
    }
    impl super::EmailVerificationService for &MockEmailVerificationService {
        async fn send_verification_email(
            &self,
            to: super::EmailAddress,
            token: &str,
        ) -> Result<(), super::EmailServiceError> {
            (*self).send_verification_email(to, token).await
        }
    }
}
