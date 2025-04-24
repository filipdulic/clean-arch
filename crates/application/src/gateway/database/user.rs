use ca_domain::entity::user::*;
use serde::Serialize;
use std::{future::Future, sync::Arc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GetError {
    #[error("User not found")]
    NotFound,
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum SaveError {
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum GetAllError {
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("User not found")]
    NotFound,
    #[error("User repository connection problem")]
    Connection,
}

#[derive(Debug, Serialize, Clone)]
pub struct Record {
    pub user: User,
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.user.id() == other.user.id()
    }
}

impl From<User> for Record {
    fn from(user: User) -> Self {
        Self { user }
    }
}

impl From<Record> for User {
    fn from(record: Record) -> Self {
        record.user
    }
}
#[cfg_attr(test, mockall::automock(type Transaction = ();))]
pub trait Repo: Send + Sync {
    type Transaction;
    fn save(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        record: Record,
    ) -> impl Future<Output = Result<(), SaveError>>;
    fn get(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        id: Id,
    ) -> impl Future<Output = Result<Record, GetError>>;
    fn get_by_username(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        username: UserName,
    ) -> impl Future<Output = Result<Record, GetError>>;
    fn get_all(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
    ) -> impl Future<Output = Result<Vec<Record>, GetAllError>>;
    fn delete(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        id: Id,
    ) -> impl Future<Output = Result<(), DeleteError>>;
}
#[cfg(test)]
impl Repo for &MockRepo {
    type Transaction = ();
    fn save(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        record: Record,
    ) -> impl Future<Output = Result<(), SaveError>> {
        (*self).save(transaction, record)
    }
    fn get(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        id: Id,
    ) -> impl Future<Output = Result<Record, GetError>> {
        (*self).get(transaction, id)
    }
    fn get_by_username(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        username: UserName,
    ) -> impl Future<Output = Result<Record, GetError>> {
        (*self).get_by_username(transaction, username)
    }
    fn get_all(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
    ) -> impl Future<Output = Result<Vec<Record>, GetAllError>> {
        (*self).get_all(transaction)
    }
    fn delete(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        id: Id,
    ) -> impl Future<Output = Result<(), DeleteError>> {
        (*self).delete(transaction, id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ca_domain::{
        entity::user::{Email, Id, Password, User, UserName},
        value_object::Role,
    };
    use rstest::rstest;

    #[rstest]
    async fn test_mock() {
        // Create a mock instance
        let mut mock = MockRepo::new();

        // Define a sample record
        let record = Record {
            user: User::new(
                Id::from(uuid::Uuid::new_v4()),
                Role::User,
                Email::new("test@email.com"),
                UserName::new("test_user"),
                Password::new("password"),
            ),
        };
        let eq_record = record.clone();

        // Set up expectations
        mock.expect_save()
            .withf(move |transaction, actual_record| {
                transaction.is_none() && actual_record == &eq_record
            })
            .times(1)
            .returning(|_, _| Box::pin(async { Ok(()) }));

        // Call the method
        let result = mock.save(None, record).await;

        // Verify the result
        assert!(result.is_ok());
    }
}
