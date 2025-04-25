use ca_application::gateway::database::{
    identifier::{NewId, NewIdError},
    signup_process::{DeleteError, GetError, Record, Repo, SaveError},
};
use ca_domain::entity::signup_process::Id;

use crate::{
    models::signup_process_state::{from_chain, SignupProcessState},
    SqlxSqlite, SqlxSqliteTransaction,
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
        println!("Save Latest State: {:?}", record);
        let sps = SignupProcessState::from(record);
        let query = sqlx::query("INSERT INTO signup_process_states (id, username, email, password, error, state) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(sps.signup_id)
            .bind(sps.username)
            .bind(sps.email)
            .bind(sps.password)
            .bind(sps.error)
            .bind(sps.state);
        let res = match transaction {
            Some(tx) => query
                .execute(&mut **tx)
                .await
                .map_err(|_| SaveError::Connection),
            None => query
                .execute(self.pool())
                .await
                .map_err(|_| SaveError::Connection),
        };
        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Error saving signup process state: {:?}", err);
                Err(SaveError::Connection)
            }
        }
    }

    async fn get_latest_state<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Record, GetError> {
        // TODO: Handle empty state chain/None
        let records = self.get_state_chain(transaction, id).await?;
        Ok(records.last().unwrap().clone())
    }

    async fn get_state_chain<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Vec<Record>, GetError> {
        let query =
            sqlx::query_as::<_, SignupProcessState>("SELECT id, username, email, password, error, state, entered_at FROM signup_process_states WHERE id = ?")
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

        Ok(from_chain(sps_results))
    }

    async fn delete<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<(), DeleteError> {
        let query =
            sqlx::query("DELETE FROM signup_process_states WHERE id = ?").bind(id.to_string());
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

impl NewId<Id> for &SqlxSqlite {
    async fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id_inner()?;
        Ok(Id::from(id))
    }
}
