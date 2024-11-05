use super::email::{EmailAddress, EmailServiceError};

pub trait EmailVerificationService {
    fn send_verification_email(
        &self,
        to: EmailAddress,
        token: &str,
    ) -> Result<(), EmailServiceError>;
}
