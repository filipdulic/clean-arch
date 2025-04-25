use async_trait::async_trait;

#[cfg(test)]
use mockall::automock;
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

#[cfg_attr(test, automock)]
#[async_trait]
pub trait EmailService {
    async fn send_email(
        &self,
        to: EmailAddress,
        subject: &str,
        body: &str,
    ) -> Result<(), EmailServiceError>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait EmailVerificationService {
    async fn send_verification_email(
        &self,
        to: EmailAddress,
        token: &str,
    ) -> Result<(), EmailServiceError>;
}

#[cfg(test)]
#[async_trait]
impl EmailVerificationService for &MockEmailVerificationService {
    async fn send_verification_email(
        &self,
        to: EmailAddress,
        token: &str,
    ) -> Result<(), EmailServiceError> {
        (*self).send_verification_email(to, token).await
    }
}
