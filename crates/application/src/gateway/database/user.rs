use std::future::Future;

use ca_domain::entity::user::*;
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
pub trait Repo: Send + Sync {
    type Transaction;
    fn save(
        &self,
        transaction: Option<&mut Self::Transaction>,
        record: Record,
    ) -> impl Future<Output = Result<(), SaveError>>;
    fn get(
        &self,
        transaction: Option<&mut Self::Transaction>,
        id: Id,
    ) -> impl Future<Output = Result<Record, GetError>>;
    fn get_by_username(
        &self,
        transaction: Option<&mut Self::Transaction>,
        username: UserName,
    ) -> impl Future<Output = Result<Record, GetError>>;
    fn get_all(
        &self,
        transaction: Option<&mut Self::Transaction>,
    ) -> impl Future<Output = Result<Vec<Record>, GetAllError>>;
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
        pub UserRepo {}
        impl Repo for  UserRepo{
            type Transaction = ();
            fn save<'a>(
                &self,
                transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
                record: Record,
            ) -> impl Future<Output = Result<(), SaveError>>;
            fn get<'a>(
                &self,
                transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
                id: Id,
            ) -> impl Future<Output = Result<Record, GetError>>;
            fn get_by_username<'a>(
                &self,
                transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
                username: UserName,
            ) -> impl Future<Output = Result<Record, GetError>>;
            fn get_all<'a>(
                &self,
                transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
            ) -> impl Future<Output = Result<Vec<Record>, GetAllError>>;
            fn delete<'a>(
                &self,
                transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
                id: Id,
            ) -> impl Future<Output = Result<(), DeleteError>>;
        }
    }
    impl Repo for &MockUserRepo {
        type Transaction = ();
        fn save<'a>(
            &self,
            transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
            record: Record,
        ) -> impl Future<Output = Result<(), SaveError>> {
            (*self).save(transaction, record)
        }
        fn get<'a>(
            &self,
            transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
            id: Id,
        ) -> impl Future<Output = Result<Record, GetError>> {
            (*self).get(transaction, id)
        }
        fn get_by_username<'a>(
            &self,
            transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
            username: UserName,
        ) -> impl Future<Output = Result<Record, GetError>> {
            (*self).get_by_username(transaction, username)
        }
        fn get_all<'a>(
            &self,
            transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
        ) -> impl Future<Output = Result<Vec<Record>, GetAllError>> {
            (*self).get_all(transaction)
        }
        fn delete<'a>(
            &self,
            transaction: Option<&'a mut <MockUserRepo as Repo>::Transaction>,
            id: Id,
        ) -> impl Future<Output = Result<(), DeleteError>> {
            (*self).delete(transaction, id)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ca_domain::{
        entity::user::{Email, Id, Password, User, UserName},
        value_object::Role,
    };

    #[tokio::test]
    async fn test_mock() {
        // Create a mock instance
        let mut mock = mock::MockUserRepo::new();

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
