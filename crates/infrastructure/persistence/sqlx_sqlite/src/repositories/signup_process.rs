use ca_application::gateway::database::{
    identifier::{NewId, NewIdError},
    signup_process::{DeleteError, GetError, Record, Repo, SaveError},
};
use ca_domain::entity::signup_process::Id;

use crate::{
    SqlxSqlite, SqlxSqliteTransaction,
    models::signup_process_state::{SignupProcessState, from_chain},
};
use sqlx;
#[async_trait::async_trait]
impl Repo for &SqlxSqlite {
    type Transaction = SqlxSqliteTransaction;
    async fn save_latest_state<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        record: Record,
    ) -> Result<(), SaveError> {
        let sps = SignupProcessState::from(record);
        let query = sqlx::query("INSERT INTO signup_process_states (id, username, email, password, error, state, entered_at) VALUES (?, ?, ?, ?, ?, ?, ?)")
            .bind(sps.signup_id)
            .bind(sps.username)
            .bind(sps.email)
            .bind(sps.password)
            .bind(sps.error)
            .bind(sps.state)
            .bind(sps.entered_at);
        match transaction {
            Some(tx) => query
                .execute(&mut **tx)
                .await
                .map_err(|_| SaveError::Connection)?,
            None => query
                .execute(self.pool())
                .await
                .map_err(|_| SaveError::Connection)?,
        };
        Ok(())
    }

    async fn get_latest_state<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Record, GetError> {
        let records = self.get_state_chain(transaction, id).await?;
        records.last().cloned().ok_or(GetError::NotFound)
    }

    async fn get_state_chain<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Vec<Record>, GetError> {
        let query =
            sqlx::query_as::<_, SignupProcessState>("SELECT id, username, email, password, error, state, entered_at FROM signup_process_states WHERE id = ? ORDER BY history_id ASC")
                .bind(id.to_string());
        let sps_results = match transaction {
            Some(tx) => query
                .fetch_all(&mut **tx)
                .await
                .map_err(|_| GetError::Connection)?,
            None => query
                .fetch_all(self.pool())
                .await
                .map_err(|_| GetError::Connection)?,
        };
        if sps_results.is_empty() {
            return Err(GetError::NotFound);
        }

        from_chain(sps_results)
    }

    async fn delete<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<(), DeleteError> {
        let query =
            sqlx::query("DELETE FROM signup_process_states WHERE id = ?").bind(id.to_string());
        let result = match transaction {
            Some(tx) => query
                .execute(&mut **tx)
                .await
                .map_err(|_| DeleteError::Connection)?,
            None => query
                .execute(self.pool())
                .await
                .map_err(|_| DeleteError::Connection)?,
        };
        if result.rows_affected() == 0 {
            return Err(DeleteError::NotFound);
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl NewId<Id> for &SqlxSqlite {
    async fn new_id(&self) -> Result<Id, NewIdError> {
        Ok(Id::from(self.new_id_inner()))
    }
}
