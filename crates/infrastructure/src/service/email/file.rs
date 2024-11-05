use ca_application::gateway::service::email::{
    EmailAddress, EmailService, EmailServiceError, EmailVerificationService,
};
use std::io::Write;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileEmailService {
    folder_path: PathBuf,
}

impl FileEmailService {
    pub fn try_new(folder_path: PathBuf) -> Result<Self, std::io::Error> {
        // let folder_path = folder_path.join("/emails");
        // TODO: user ref or AsRef?
        std::fs::create_dir_all(folder_path.clone())?;

        Ok(Self { folder_path })
    }
}

impl EmailService for FileEmailService {
    fn send_email(
        &self,
        to: EmailAddress,
        subject: &str,
        body: &str,
    ) -> Result<(), EmailServiceError> {
        let file_name = format!("{}.txt", to.as_str());
        let file_path = self.folder_path.join(file_name);

        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
            .map_err(|_| EmailServiceError::SendEmailFailed)?;

        writeln!(file, "Subject: {}", subject).map_err(|_| EmailServiceError::SendEmailFailed)?;
        writeln!(file, "Body: {}", body).map_err(|_| EmailServiceError::SendEmailFailed)?;

        Ok(())
    }
}

impl EmailVerificationService for FileEmailService {
    fn send_verification_email(
        &self,
        to: EmailAddress,
        verification_code: &str,
    ) -> Result<(), EmailServiceError> {
        let subject = "Please verify your email address";
        let body = format!("Your verification code is: `{}`", verification_code);

        self.send_email(to, subject, &body)
    }
}
