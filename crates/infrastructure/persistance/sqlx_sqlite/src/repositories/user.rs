use ca_application::{
    gateway::repository::user::{DeleteError, GetAllError, GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use ca_domain::entity::user::{Id, UserName};

use crate::{models::user::User, SqlxSqlite};

impl Repo for &SqlxSqlite {
    async fn save(&self, record: Record) -> Result<(), SaveError> {
        sqlx::query("INSERT INTO users (id, name, email, password, role) VALUES (?, ?, ?, ?, ?)")
            .bind(record.user.id().to_string())
            .bind(record.user.username().to_string())
            .bind(record.user.email().to_string())
            .bind(record.user.password().to_string())
            .bind(record.user.role().to_string())
            .execute(self.pool())
            .await
            .map_err(|_| SaveError::Connection)?;
        Ok(())
    }

    async fn get(&self, id: Id) -> Result<Record, GetError> {
        let user_results = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role FROM users WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(self.pool())
        .await
        .map_err(|_| GetError::Connection)?
        .ok_or(GetError::NotFound)?;
        Ok(Record::from(user_results))
    }

    async fn get_by_username(&self, username: UserName) -> Result<Record, GetError> {
        let user_result = sqlx::query_as::<_, User>(
            "SELECT id, name, email, password, role FROM users WHERE name = ?",
        )
        .bind(username.to_string())
        .fetch_optional(self.pool())
        .await
        .map_err(|_| GetError::Connection)?
        .ok_or(GetError::NotFound)?;
        Ok(Record::from(user_result))
    }

    async fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
        let user_results =
            sqlx::query_as::<_, User>("SELECT id, name, email, password, role FROM users")
                .fetch_all(self.pool())
                .await
                .map_err(|_| GetAllError::Connection)?;
        Ok(user_results.into_iter().map(Record::from).collect())
    }

    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id.to_string())
            .execute(self.pool())
            .await
            .map_err(|_| DeleteError::Connection)?;
        Ok(())
    }
}

impl NewId<Id> for &SqlxSqlite {
    async fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id_inner()?;
        Ok(Id::from(id))
    }
}
