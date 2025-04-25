use futures::lock::Mutex;
use std::sync::Arc;

use ca_application::gateway::database::user::{
    DeleteError, GetAllError, GetError, Record, Repo, SaveError,
};
use ca_domain::entity::user::{Id, UserName};

use crate::{models::user::User, SqlxSqliteTransaction};

use super::SqlxSqliteRepository;
#[async_trait::async_trait]

impl Repo for SqlxSqliteRepository {
    type Transaction = SqlxSqliteTransaction;
    async fn save(
        &self,
        transaction: Option<Arc<Mutex<Self::Transaction>>>,
        record: Record,
    ) -> Result<(), SaveError> {
        let query = sqlx::query(
            "INSERT INTO users (id, name, email, password, role) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(record.user.id().to_string())
        .bind(record.user.username().to_string())
        .bind(record.user.email().to_string())
        .bind(record.user.password().to_string())
        .bind(record.user.role().to_string());
        match transaction {
            Some(tx) => {
                query
                    .execute(&mut **tx.lock().await)
                    .await
                    .map_err(|_| SaveError::Connection)?;
            }
            None => {
                query
                    .execute(self.pool.as_ref())
                    .await
                    .map_err(|_| SaveError::Connection)?;
            }
        };
        Ok(())
    }

    async fn get(
        &self,
        transaction: Option<Arc<Mutex<Self::Transaction>>>,
        id: Id,
    ) -> Result<Record, GetError> {
        let query = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role FROM users WHERE id = ?",
        )
        .bind(id.to_string());
        let user_result = match transaction {
            Some(tx) => query
                .fetch_optional(&mut **tx.lock().await)
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
            None => query
                .fetch_optional(self.pool.as_ref())
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
        };
        Ok(Record::from(user_result))
    }

    async fn get_by_username(
        &self,
        transaction: Option<Arc<Mutex<Self::Transaction>>>,
        username: UserName,
    ) -> Result<Record, GetError> {
        let query = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role FROM users WHERE name = ?",
        )
        .bind(username.to_string());
        let user_result = match transaction {
            Some(tx) => query
                .fetch_optional(&mut **tx.lock().await)
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
            None => query
                .fetch_optional(self.pool.as_ref())
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
        };
        Ok(Record::from(user_result))
    }

    async fn get_all(
        &self,
        transaction: Option<Arc<Mutex<Self::Transaction>>>,
    ) -> Result<Vec<Record>, GetAllError> {
        let query = sqlx::query_as::<_, User>("SELECT id, name, email, password, role FROM users");
        let user_results = match transaction {
            Some(tx) => query
                .fetch_all(&mut **tx.lock().await)
                .await
                .map_err(|_| GetAllError::Connection)?,
            None => query
                .fetch_all(self.pool.as_ref())
                .await
                .map_err(|_| GetAllError::Connection)?,
        };
        Ok(user_results.into_iter().map(Record::from).collect())
    }

    async fn delete(
        &self,
        transaction: Option<Arc<Mutex<Self::Transaction>>>,
        id: Id,
    ) -> Result<(), DeleteError> {
        let query = sqlx::query("DELETE FROM users WHERE id = ?").bind(id.to_string());
        match transaction {
            Some(tx) => query
                .execute(&mut **tx.lock().await)
                .await
                .map_err(|_| DeleteError::Connection)?,
            None => query
                .execute(self.pool.as_ref())
                .await
                .map_err(|_| DeleteError::Connection)?,
        };
        Ok(())
    }
}
