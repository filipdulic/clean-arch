use super::JsonFile;
use ca_application::gateway::repository::token::*;
use std::io;
use uuid;

impl Repo for JsonFile {
    fn gen(&self, email: &str) -> Result<Record, GenError> {
        log::debug!("Generate token for email: {}", email);
        let token = uuid::Uuid::new_v4().to_string();
        self.tokens.save_with_id(&token, email).map_err(|_| {
            log::warn!("Unable to save Token!");
            GenError::Connection
        })?;
        Ok(Record { token })
    }
    fn verify(&self, email: &str, token: &str) -> Result<(), VerifyError> {
        log::debug!("Verify token for email: {} and token: {}", email, token);
        let stored_token = self.tokens.get::<String>(email).map_err(|err| {
            log::warn!("Unable to fetch token: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                VerifyError::NotFound
            } else {
                VerifyError::Connection
            }
        })?;
        if stored_token != token {
            log::warn!("Token mismatch: {} != {}", stored_token, token);
            return Err(VerifyError::Mismatch);
        }
        Ok(())
    }
}
