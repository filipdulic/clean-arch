use super::{models::VerificationToken, JsonFile};
use ca_application::gateway::repository::token::*;
use std::io;

use chrono::{Duration, Utc};

impl Repo for &JsonFile {
    async fn gen(&self, email: &str) -> Result<Record, GenError> {
        log::debug!("Generate token for email: {}", email);
        let token = VerificationToken {
            token: uuid::Uuid::new_v4(),
            created_at: Utc::now(),
        };
        self.tokens.save_with_id(&token, email).map_err(|_| {
            log::warn!("Unable to save Token!");
            GenError::Connection
        })?;
        Ok(Record {
            token: token.token.to_string(),
        })
    }
    async fn verify(&self, email: &str, token: &str) -> Result<(), VerifyError> {
        log::debug!("Verify token for email: {} and token: {}", email, token);
        let stored_token = self.tokens.get::<VerificationToken>(email).map_err(|err| {
            log::warn!("Unable to fetch token: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                VerifyError::NotFound
            } else {
                VerifyError::Connection
            }
        })?;
        if stored_token.token.to_string() != token {
            log::warn!("Token mismatch!");
            return Err(VerifyError::Mismatch);
        }
        if Utc::now() - stored_token.created_at > Duration::days(1) {
            log::warn!("Token expired!");
            return Err(VerifyError::TokenExpired);
        }
        Ok(())
    }
    async fn extend(&self, email: &str) -> Result<(), ExtendError> {
        log::debug!("Extend token for email: {}", email);
        let mut stored_token = self.tokens.get::<VerificationToken>(email).map_err(|err| {
            log::warn!("Unable to fetch token: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                ExtendError::NotFound
            } else {
                ExtendError::Connection
            }
        })?;
        stored_token.created_at = Utc::now();
        self.tokens
            .save_with_id(&stored_token, email)
            .map_err(|_| {
                log::warn!("Unable to save Token!");
                ExtendError::Connection
            })?;
        Ok(())
    }
}
