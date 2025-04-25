use ca_application::gateway::database::user::{
    DeleteError, GetAllError, GetError, Record, Repo, SaveError,
};
use ca_domain::entity::user::{Id, UserName};

use crate::{models::user::User, SqlxSqlite, SqlxSqliteTransaction};
#[async_trait::async_trait]
impl Repo for &SqlxSqlite {
    type Transaction = SqlxSqliteTransaction;
    async fn save<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
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
                    .execute(&mut **tx)
                    .await
                    .map_err(|_| SaveError::Connection)?;
            }
            None => {
                query
                    .execute(self.pool())
                    .await
                    .map_err(|_| SaveError::Connection)?;
            }
        };
        Ok(())
    }

    async fn get<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Record, GetError> {
        let query = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role FROM users WHERE id = ?",
        )
        .bind(id.to_string());
        let user_result = match transaction {
            Some(tx) => query
                .fetch_optional(&mut **tx)
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
            None => query
                .fetch_optional(self.pool())
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
        };
        Ok(Record::from(user_result))
    }

    async fn get_by_username<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        username: UserName,
    ) -> Result<Record, GetError> {
        let query = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role FROM users WHERE name = ?",
        )
        .bind(username.to_string());
        let user_result = match transaction {
            Some(tx) => query
                .fetch_optional(&mut **tx)
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
            None => query
                .fetch_optional(self.pool())
                .await
                .map_err(|_| GetError::Connection)?
                .ok_or(GetError::NotFound)?,
        };
        Ok(Record::from(user_result))
    }

    async fn get_all<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
    ) -> Result<Vec<Record>, GetAllError> {
        let query = sqlx::query_as::<_, User>("SELECT id, name, email, password, role FROM users");
        let user_results = match transaction {
            Some(tx) => query
                .fetch_all(&mut **tx)
                .await
                .map_err(|_| GetAllError::Connection)?,
            None => query
                .fetch_all(self.pool())
                .await
                .map_err(|_| GetAllError::Connection)?,
        };
        Ok(user_results.into_iter().map(Record::from).collect())
    }

    async fn delete<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<(), DeleteError> {
        let query = sqlx::query("DELETE FROM users WHERE id = ?").bind(id.to_string());
        match transaction {
            Some(tx) => query
                .execute(&mut **tx)
                .await
                .map_err(|_| DeleteError::Connection)?,
            None => query
                .execute(self.pool())
                .await
                .map_err(|_| DeleteError::Connection)?,
        };
        Ok(())
    }
}
