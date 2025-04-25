use async_trait::async_trait;
use ca_domain::{entity::signup_process::SignupProcessValue, value_object::Id};

use identifier::NewId;
#[cfg(test)]
use identifier::NewIdError;

pub mod identifier;
pub mod signup_process;
pub mod token;
pub mod user;

#[cfg(test)]
use mockall::mock;

#[async_trait]
pub trait Database {
    type Transaction;
    type Error;
    fn signup_process_repo(&self) -> impl signup_process::Repo<Transaction = Self::Transaction>;
    fn signuo_id_gen(&self) -> impl NewId<Id<SignupProcessValue>>;
    fn user_repo(&self) -> impl user::Repo<Transaction = Self::Transaction>;
    fn token_repo(&self) -> impl token::Repo<Transaction = Self::Transaction>;
    async fn begin_transaction(&self) -> Self::Transaction;
    async fn commit_transaction(&self, transaction: Self::Transaction) -> Result<(), Self::Error>;
    async fn rollback_transaction(&self, transaction: Self::Transaction)
        -> Result<(), Self::Error>;
}

#[cfg(test)]
mock! {
    pub SignupIdGen {}
    #[async_trait]
    impl NewId<Id<SignupProcessValue>> for SignupIdGen {
        async fn new_id(&self) -> Result<Id<SignupProcessValue>, NewIdError>;
    }
}
#[cfg(test)]
#[async_trait]
impl NewId<Id<SignupProcessValue>> for &MockSignupIdGen {
    async fn new_id(&self) -> Result<Id<SignupProcessValue>, NewIdError> {
        (**self).new_id().await
    }
}
#[cfg(test)]
pub struct MockDatabase {
    pub signup_process_repo: signup_process::MockRepo,
    pub signup_id_gen: MockSignupIdGen,
    pub token_repo: token::MockRepo,
    pub user_repo: user::MockRepo,
}
#[cfg(test)]
impl Default for MockDatabase {
    fn default() -> Self {
        Self {
            signup_process_repo: signup_process::MockRepo::new(),
            signup_id_gen: MockSignupIdGen::new(),
            token_repo: token::MockRepo::new(),
            user_repo: user::MockRepo::new(),
        }
    }
}
#[cfg(test)]
#[async_trait]
impl Database for &MockDatabase {
    type Transaction = ();
    type Error = ();
    fn signup_process_repo(&self) -> impl signup_process::Repo<Transaction = Self::Transaction> {
        &self.signup_process_repo
    }
    fn signuo_id_gen(&self) -> impl NewId<Id<SignupProcessValue>> {
        &self.signup_id_gen
    }
    fn user_repo(&self) -> impl user::Repo<Transaction = Self::Transaction> {
        &self.user_repo
    }
    fn token_repo(&self) -> impl token::Repo<Transaction = Self::Transaction> {
        &self.token_repo
    }
    async fn begin_transaction(&self) -> Self::Transaction {}
    async fn commit_transaction(&self, _transaction: Self::Transaction) -> Result<(), Self::Error> {
        Ok(())
    }
    async fn rollback_transaction(
        &self,
        _transaction: Self::Transaction,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
