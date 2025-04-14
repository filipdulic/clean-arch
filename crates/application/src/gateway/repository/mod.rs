use std::future::Future;

use ca_domain::{entity::signup_process::SignupProcessValue, value_object::Id};

use crate::identifier::NewId;

pub mod signup_process;
pub mod token;
pub mod user;

pub trait Database {
    type Transaction;
    type Error;
    fn signup_process_repo(&self) -> impl signup_process::Repo<Transaction = Self::Transaction>;
    fn signuo_id_gen(&self) -> impl NewId<Id<SignupProcessValue>>;
    fn user_repo(&self) -> impl user::Repo<Transaction = Self::Transaction>;
    fn token_repo(&self) -> impl token::Repo<Transaction = Self::Transaction>;
    fn begin_transaction(&self) -> impl Future<Output = Self::Transaction>;
    fn commit_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<(), Self::Error>>;
    fn rollback_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}
