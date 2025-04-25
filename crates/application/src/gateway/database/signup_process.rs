use async_trait::async_trait;
use ca_domain::entity::signup_process::*;
use chrono::{DateTime, Utc};
#[cfg(test)]
use mockall::automock;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum GetError {
    #[error("SignupProcess not found")]
    NotFound,
    #[error("SignupProcess repository connection problem")]
    Connection,
    #[error("SignupProcess in incorrect state")]
    IncorrectState,
}

#[derive(Debug, Error, Serialize)]
pub enum SaveError {
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Error, Serialize)]
pub enum DeleteError {
    #[error("SignupProcess not found")]
    NotFound,
    #[error("SignupProcess repository connection problem")]
    Connection,
}

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub id: Id,
    pub state: SignupStateEnum,
    pub entered_at: DateTime<Utc>,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Record {}

impl<S: SignupStateTrait> From<SignupProcess<S>> for Record {
    fn from(process: SignupProcess<S>) -> Self {
        Record {
            id: process.id(),
            state: process.state().clone().into(),
            entered_at: process.entered_at(),
        }
    }
}

impl<S: SignupStateTrait + Clone> TryFrom<Record> for SignupProcess<S> {
    type Error = GetError;
    fn try_from(value: Record) -> Result<Self, Self::Error> {
        (value.id, value.state, value.entered_at)
            .try_into()
            .map_err(|_| GetError::IncorrectState)
    }
}
#[cfg_attr(test, automock(type Transaction = ();))]
#[async_trait]
pub trait Repo: Send + Sync {
    type Transaction;
    async fn save_latest_state<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        record: Record,
    ) -> Result<(), SaveError>;
    async fn get_latest_state<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Record, GetError>;
    async fn get_state_chain<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Vec<Record>, GetError>;
    async fn delete<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<(), DeleteError>;
}

#[cfg(test)]
#[async_trait]
impl Repo for &MockRepo {
    type Transaction = ();
    async fn save_latest_state<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        record: Record,
    ) -> Result<(), SaveError> {
        (**self).save_latest_state(transaction, record).await
    }
    async fn get_latest_state<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Record, GetError> {
        (**self).get_latest_state(transaction, id).await
    }
    async fn get_state_chain<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Vec<Record>, GetError> {
        (**self).get_state_chain(transaction, id).await
    }
    async fn delete<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<(), DeleteError> {
        (**self).delete(transaction, id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ca_domain::entity::signup_process::Id;
    use rstest::rstest;

    #[rstest]
    async fn test_mock() {
        // Create a mock instance
        let mut mock = MockRepo::new();

        // Define a sample record
        let record = Record {
            id: Id::new(uuid::Uuid::new_v4()), // Assuming Id::new() creates a new ID
            state: SignupStateEnum::ForDeletion, // Adjust based on your SignupStateEnum
            entered_at: Utc::now(),
        };
        let eq_record = record.clone();

        // Set up expectations
        mock.expect_save_latest_state()
            .withf(move |transaction, actual_record| {
                transaction.is_none() && actual_record == &eq_record
            })
            .times(1)
            .returning(|_, _| Ok(()));

        // Call the method
        let result = mock.save_latest_state(None, record).await;

        // Verify the result
        assert!(result.is_ok());
    }
}
