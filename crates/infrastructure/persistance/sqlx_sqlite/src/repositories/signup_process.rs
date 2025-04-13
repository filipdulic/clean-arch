use ca_application::{
    gateway::repository::signup_process::{DeleteError, GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use ca_domain::entity::signup_process::Id;

use crate::{
    models::signup_process_state::{from_chain, SignupProcessState},
    SqlxSqlite,
};
use sqlx;

impl Repo for &SqlxSqlite {
    async fn save_latest_state(&self, record: Record) -> Result<(), SaveError> {
        println!("Save Latest State: {:?}", record);
        let sps = SignupProcessState::from(record);
        let res = sqlx::query("INSERT INTO signup_process_states (id, username, email, password, error, state) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(sps.signup_id)
            .bind(sps.username)
            .bind(sps.email)
            .bind(sps.password)
            .bind(sps.error)
            .bind(sps.state)
            .execute(self.pool())
            .await;
        match res {
            Ok(_) => Ok(()),
            Err(err) => {
                println!("Error saving signup process state: {:?}", err);
                Err(SaveError::Connection)
            }
        }
    }

    async fn get_latest_state(&self, id: Id) -> Result<Record, GetError> {
        // TODO: Handle empty state chain/None
        let records = self.get_state_chain(id).await?;
        Ok(records.last().unwrap().clone())
    }

    async fn get_state_chain(&self, id: Id) -> Result<Vec<Record>, GetError> {
        let sps_results =
            sqlx::query_as::<_, SignupProcessState>("SELECT id, username, email, password, error, state, entered_at FROM signup_process_states WHERE id = ?")
                .bind(id.to_string())
                .fetch_all(self.pool())
                .await
                .map_err(|_| GetError::Connection)?;
        Ok(from_chain(sps_results))
    }

    async fn delete(&self, id: Id) -> Result<(), DeleteError> {
        sqlx::query("DELETE FROM signup_process_states WHERE id = ?")
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
