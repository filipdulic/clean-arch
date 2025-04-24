use std::{future::Future, sync::Arc};

use ca_domain::{entity::signup_process::SignupProcessValue, value_object::Id};

use identifier::NewId;

pub mod identifier;
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
    fn begin_transaction(
        &self,
    ) -> impl Future<Output = Arc<futures::lock::Mutex<Self::Transaction>>>;
    fn commit_transaction(
        &self,
        transaction: Arc<futures::lock::Mutex<Self::Transaction>>,
    ) -> impl Future<Output = Result<(), Self::Error>>;
    fn rollback_transaction(
        &self,
        transaction: Arc<futures::lock::Mutex<Self::Transaction>>,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}

#[cfg(test)]
pub mod mock {
    use std::sync::Arc;

    use super::*;
    use crate::gateway::database::{
        identifier::{NewId, NewIdError},
        signup_process::MockRepo as MockSignupProcessRepo,
        token::MockRepo as MockTokenRepo,
        user::MockRepo as MockUserRepo,
    };
    use ca_domain::{entity::signup_process::SignupProcessValue, value_object::Id};
    use mockall::mock;

    mock! {
        pub SignupIdGen {}
        impl NewId<Id<SignupProcessValue>> for SignupIdGen {
            fn new_id(&self) -> impl Future<Output = Result<Id<SignupProcessValue>, NewIdError>>;
        }
    }
    impl NewId<Id<SignupProcessValue>> for &MockSignupIdGen {
        fn new_id(&self) -> impl Future<Output = Result<Id<SignupProcessValue>, NewIdError>> {
            (*self).new_id()
        }
    }
    pub struct MockDatabase {
        pub signup_process_repo: MockSignupProcessRepo,
        pub signup_id_gen: MockSignupIdGen,
        pub token_repo: MockTokenRepo,
        pub user_repo: MockUserRepo,
    }
    impl Default for MockDatabase {
        fn default() -> Self {
            Self {
                signup_process_repo: MockSignupProcessRepo::new(),
                signup_id_gen: MockSignupIdGen::new(),
                token_repo: MockTokenRepo::new(),
                user_repo: MockUserRepo::new(),
            }
        }
    }
    impl Database for &MockDatabase {
        type Transaction = ();
        type Error = ();
        fn signup_process_repo(
            &self,
        ) -> impl signup_process::Repo<Transaction = Self::Transaction> {
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
        async fn begin_transaction(&self) -> Arc<futures::lock::Mutex<Self::Transaction>> {
            Arc::new(futures::lock::Mutex::new(()))
        }
        async fn commit_transaction(
            &self,
            _transaction: Arc<futures::lock::Mutex<Self::Transaction>>,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
        async fn rollback_transaction(
            &self,
            _transaction: Arc<futures::lock::Mutex<Self::Transaction>>,
        ) -> Result<(), Self::Error> {
            Ok(())
        }
    }
}
