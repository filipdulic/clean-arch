use std::sync::Arc;

use ca_domain::{entity::signup_process::SignupProcessValue, value_object::Id};

use identifier::NewId;

pub mod identifier;
pub mod signup_process;
pub mod token;
pub mod user;

#[async_trait::async_trait]
pub trait Database {
    type Transaction;
    type Error;
    fn signup_process_repo(
        &self,
    ) -> Arc<dyn signup_process::Repo<Transaction = Self::Transaction> + Send + Sync>;
    fn signuo_id_gen(&self) -> Arc<dyn NewId<Id<SignupProcessValue>> + Send + Sync>;
    fn user_repo(&self) -> Arc<dyn user::Repo<Transaction = Self::Transaction> + Send + Sync>;
    fn token_repo(&self) -> Arc<dyn token::Repo<Transaction = Self::Transaction> + Send + Sync>;
    async fn begin_transaction(&self) -> Arc<futures::lock::Mutex<Self::Transaction>>;
    async fn commit_transaction(
        &self,
        transaction: Arc<futures::lock::Mutex<Self::Transaction>>,
    ) -> Result<(), Self::Error>;
    async fn rollback_transaction(
        &self,
        transaction: Arc<futures::lock::Mutex<Self::Transaction>>,
    ) -> Result<(), Self::Error>;
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
        #[async_trait::async_trait]
        impl NewId<Id<SignupProcessValue>> for SignupIdGen {
            async fn new_id(&self) -> Result<Id<SignupProcessValue>, NewIdError>;
        }
    }
    #[async_trait::async_trait]
    impl NewId<Id<SignupProcessValue>> for &MockSignupIdGen {
        async fn new_id(&self) -> Result<Id<SignupProcessValue>, NewIdError> {
            (*self).new_id().await
        }
    }
    pub struct MockDatabase {
        pub signup_process_repo: Arc<MockSignupProcessRepo>,
        pub signup_id_gen: Arc<MockSignupIdGen>,
        pub token_repo: Arc<MockTokenRepo>,
        pub user_repo: Arc<MockUserRepo>,
    }
    impl MockDatabase {
        pub fn new(
            signup_process_repo: MockSignupProcessRepo,
            signup_id_gen: MockSignupIdGen,
            token_repo: MockTokenRepo,
            user_repo: MockUserRepo,
        ) -> Self {
            Self {
                signup_process_repo: Arc::new(signup_process_repo),
                signup_id_gen: Arc::new(signup_id_gen),
                token_repo: Arc::new(token_repo),
                user_repo: Arc::new(user_repo),
            }
        }
    }
    #[async_trait::async_trait]
    impl Database for MockDatabase {
        type Transaction = ();
        type Error = ();
        fn signup_process_repo(
            &self,
        ) -> Arc<dyn signup_process::Repo<Transaction = Self::Transaction> + Send + Sync> {
            self.signup_process_repo.clone()
        }
        fn signuo_id_gen(&self) -> Arc<dyn NewId<Id<SignupProcessValue>> + Send + Sync> {
            self.signup_id_gen.clone()
        }
        fn user_repo(&self) -> Arc<dyn user::Repo<Transaction = Self::Transaction> + Send + Sync> {
            self.user_repo.clone()
        }
        fn token_repo(
            &self,
        ) -> Arc<dyn token::Repo<Transaction = Self::Transaction> + Send + Sync> {
            self.token_repo.clone()
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
