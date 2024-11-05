pub struct EmailAddress(String);
impl EmailAddress {
    pub fn new(address: &str) -> Self {
        Self(address.to_string())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
#[derive(Debug)]
pub enum EmailServiceError {
    InvalidEmailAddress,
    SendEmailFailed,
}
pub trait EmailService {
    fn send_email(
        &self,
        to: EmailAddress,
        subject: &str,
        body: &str,
    ) -> Result<(), EmailServiceError>;
}

pub trait EmailVerificationService {
    fn send_verification_email(
        &self,
        to: EmailAddress,
        token: &str,
    ) -> Result<(), EmailServiceError>;
}