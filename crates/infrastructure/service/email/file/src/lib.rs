use ca_application::gateway::service::email::{
    EmailAddress, EmailService, EmailServiceError, EmailVerificationService,
};
use directories::UserDirs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileEmailService {
    folder_path: PathBuf,
}

impl FileEmailService {
    pub fn try_new(folder_path: PathBuf) -> Result<Self, std::io::Error> {
        std::fs::create_dir_all(folder_path.clone())?;

        Ok(Self { folder_path })
    }
}
// TODO:use async file system
#[async_trait::async_trait]
impl EmailService for &FileEmailService {
    async fn send_email(
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

#[async_trait::async_trait]
impl EmailVerificationService for &FileEmailService {
    async fn send_verification_email(
        &self,
        to: EmailAddress,
        verification_code: &str,
    ) -> Result<(), EmailServiceError> {
        let subject = "Please verify your email address";
        let body = format!("Your verification code is: `{}`", verification_code);

        self.send_email(to, subject, &body).await
    }
}

const DEFAULT_STORAGE_DIR_NAME: &str = "clean-architecture-with-rust-data";

// Get storage directory with the following priority:
// 1. Custom (passed by the CLI)
// 2. HOME/DOCUMENTS/clean-architecture-with-rust-data
// 3. HOME/clean-architecture-with-rust-data
// 4. Relative to the executable: ./clean-architecture-with-rust-data
pub fn data_storage_directory(data_dir: Option<PathBuf>) -> PathBuf {
    if let Some(data_dir) = data_dir {
        data_dir
    } else {
        let base_path = if let Some(users_dir) = UserDirs::new() {
            users_dir
                .document_dir()
                .unwrap_or_else(|| users_dir.home_dir())
                .to_path_buf()
        } else {
            Path::new(".").to_path_buf()
        };
        base_path.join(DEFAULT_STORAGE_DIR_NAME)
    }
}
