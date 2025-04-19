use std::future::Future;

use ca_domain::entity::signup_process::*;
use chrono::{DateTime, Utc};
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

pub trait Repo: Send + Sync {
    type Transaction;
    fn save_latest_state(
        &self,
        transaction: Option<&mut Self::Transaction>,
        record: Record,
    ) -> impl Future<Output = Result<(), SaveError>>;
    fn get_latest_state(
        &self,
        transaction: Option<&mut Self::Transaction>,
        id: Id,
    ) -> impl Future<Output = Result<Record, GetError>>;
    fn get_state_chain(
        &self,
        transaction: Option<&mut Self::Transaction>,
        id: Id,
    ) -> impl Future<Output = Result<Vec<Record>, GetError>>;
    fn delete(
        &self,
        transaction: Option<&mut Self::Transaction>,
        id: Id,
    ) -> impl Future<Output = Result<(), DeleteError>>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use mockall::mock;
    mock! {
        pub SignupProcessRepo {}
        impl Repo for  SignupProcessRepo{
            type Transaction = ();
            fn save_latest_state<'a>(
                &self,
                transaction: Option<&'a mut <MockSignupProcessRepo as Repo>::Transaction>,
                record: Record,
            ) -> impl Future<Output = Result<(), SaveError>>;
            fn get_latest_state<'a>(
                &self,
                transaction: Option<&'a mut <MockSignupProcessRepo as Repo>::Transaction>,
                id: Id,
            ) -> impl Future<Output = Result<Record, GetError>>;
            fn get_state_chain<'a>(
                &self,
                transaction: Option<&'a mut <MockSignupProcessRepo as Repo>::Transaction>,
                id: Id,
            ) -> impl Future<Output = Result<Vec<Record>, GetError>>;
            fn delete<'a>(
                &self,
                transaction: Option<&'a mut <MockSignupProcessRepo as Repo>::Transaction>,
                id: Id,
            ) -> impl Future<Output = Result<(), DeleteError>>;
        }
    }
    impl Repo for &MockSignupProcessRepo {
        type Transaction = ();
        fn save_latest_state(
            &self,
            transaction: Option<&mut Self::Transaction>,
            record: Record,
        ) -> impl Future<Output = Result<(), SaveError>> {
            (*self).save_latest_state(transaction, record)
        }
        fn get_latest_state(
            &self,
            transaction: Option<&mut Self::Transaction>,
            id: Id,
        ) -> impl Future<Output = Result<Record, GetError>> {
            (*self).get_latest_state(transaction, id)
        }
        fn get_state_chain(
            &self,
            transaction: Option<&mut Self::Transaction>,
            id: Id,
        ) -> impl Future<Output = Result<Vec<Record>, GetError>> {
            (*self).get_state_chain(transaction, id)
        }
        fn delete(
            &self,
            transaction: Option<&mut Self::Transaction>,
            id: Id,
        ) -> impl Future<Output = Result<(), DeleteError>> {
            (*self).delete(transaction, id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ca_domain::entity::signup_process::Id;

    #[tokio::test]
    async fn test_mock() {
        // Create a mock instance
        let mut mock = mock::MockSignupProcessRepo::new();

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
            .returning(|_, _| Box::pin(async { Ok(()) }));

        // Call the method
        let result = mock.save_latest_state(None, record).await;

        // Verify the result
        assert!(result.is_ok());
    }
}
