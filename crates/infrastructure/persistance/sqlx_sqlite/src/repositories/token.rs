use ca_application::gateway::database::token::*;
use chrono::{DateTime, Duration, NaiveDateTime, Utc};

use crate::{SqlxSqlite, SqlxSqliteTransaction};

impl Repo for &SqlxSqlite {
    type Transaction = SqlxSqliteTransaction;
    async fn gen(
        &self,
        transaction: Option<&mut Self::Transaction>,
        email: &str,
    ) -> Result<Record, GenError> {
        // log::debug!("Generate token for email: {}", email);
        let token = uuid::Uuid::new_v4();
        let query = sqlx::query("INSERT INTO tokens (token, email) VALUES (?, ?)")
            .bind(token.to_string())
            .bind(email.to_string());
        match transaction {
            Some(tx) => {
                query
                    .execute(&mut **tx)
                    .await
                    .map_err(|_| GenError::Connection)?;
            }
            None => {
                query
                    .execute(self.pool())
                    .await
                    .map_err(|_| GenError::Connection)?;
            }
        };
        Ok(Record {
            token: token.to_string(),
        })
    }

    async fn verify(
        &self,
        transaction: Option<&mut Self::Transaction>,
        email: &str,
        token: &str,
    ) -> Result<(), VerifyError> {
        log::debug!("Verify token for email: {} and token: {}", email, token);
        let query = sqlx::query_as("SELECT token, email, created_at FROM tokens WHERE token = ?")
            .bind(token.to_string());
        let maybe_row: Option<(String, String, String)> = match transaction {
            Some(tx) => query
                .fetch_optional(&mut **tx)
                .await
                .map_err(|_| VerifyError::Connection)?,
            None => query
                .fetch_optional(self.pool())
                .await
                .map_err(|_| VerifyError::Connection)?,
        };
        if maybe_row.is_none() {
            log::warn!("Token not found!");
            return Err(VerifyError::NotFound);
        }
        let (_, db_email, db_created_at) = maybe_row.unwrap();

        if db_email != email {
            log::warn!("Email mismatch!");
            return Err(VerifyError::Mismatch);
        }
        let created_at = NaiveDateTime::parse_from_str(&db_created_at, "%Y-%m-%d %H:%M:%S")
            .map(|naive| DateTime::<Utc>::from_naive_utc_and_offset(naive, Utc))
            .unwrap();
        if Utc::now() - created_at > Duration::days(1) {
            log::warn!("Token expired!");
            return Err(VerifyError::TokenExpired);
        }
        Ok(())
    }

    async fn extend(
        &self,
        transaction: Option<&mut Self::Transaction>,
        email: &str,
    ) -> Result<(), ExtendError> {
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let query = sqlx::query("UPDATE tokens SET created_at = ? WHERE email = ?")
            .bind(now)
            .bind(email.to_string());
        match transaction {
            Some(tx) => query
                .execute(&mut **tx)
                .await
                .map_err(|_| ExtendError::Connection)?,
            None => query
                .execute(self.pool())
                .await
                .map_err(|_| ExtendError::Connection)?,
        };
        Ok(())
    }
}
