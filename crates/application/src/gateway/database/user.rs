use async_trait::async_trait;
use ca_domain::entity::user::*;
#[cfg(test)]
use mockall::automock;
use serde::Serialize;
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

#[cfg_attr(test, automock(type Transaction = ();))]
#[async_trait]
pub trait Repo: Send + Sync {
    type Transaction;
    async fn save<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        record: Record,
    ) -> Result<(), SaveError>;
    async fn get<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        id: Id,
    ) -> Result<Record, GetError>;
    async fn get_by_username<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
        username: UserName,
    ) -> Result<Record, GetError>;
    async fn get_all<'a>(
        &self,
        transaction: Option<&'a mut Self::Transaction>,
    ) -> Result<Vec<Record>, GetAllError>;
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
    async fn save<'a>(
        &self,
        transaction: Option<&'a mut <MockRepo as Repo>::Transaction>,
        record: Record,
    ) -> Result<(), SaveError> {
        (*self).save(transaction, record).await
    }
    async fn get<'a>(
        &self,
        transaction: Option<&'a mut <MockRepo as Repo>::Transaction>,
        id: Id,
    ) -> Result<Record, GetError> {
        (*self).get(transaction, id).await
    }
    async fn get_by_username<'a>(
        &self,
        transaction: Option<&'a mut <MockRepo as Repo>::Transaction>,
        username: UserName,
    ) -> Result<Record, GetError> {
        (*self).get_by_username(transaction, username).await
    }
    async fn get_all<'a>(
        &self,
        transaction: Option<&'a mut <MockRepo as Repo>::Transaction>,
    ) -> Result<Vec<Record>, GetAllError> {
        (*self).get_all(transaction).await
    }
    async fn delete<'a>(
        &self,
        transaction: Option<&'a mut <MockRepo as Repo>::Transaction>,
        id: Id,
    ) -> Result<(), DeleteError> {
        (*self).delete(transaction, id).await
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
            .returning(|_, _| Ok(()));

        // Call the method
        let result = mock.save(None, record).await;

        // Verify the result
        assert!(result.is_ok());
    }
}
