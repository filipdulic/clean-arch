pub struct EmailAddress(String);
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
