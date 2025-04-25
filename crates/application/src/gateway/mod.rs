use std::sync::Arc;

use database::Database;

pub mod database;
pub mod service;

pub trait DatabaseProvider {
    type Transaction;
    type Error;
    fn database(
        &self,
    ) -> Arc<dyn Database<Transaction = Self::Transaction, Error = Self::Error> + Send + Sync>;
}

pub trait EmailVerificationServiceProvider {
    fn email_verification_service(
        &self,
    ) -> Arc<dyn service::email::EmailVerificationService + Send + Sync>;
}

pub trait AuthPackerProvider {
    fn auth_packer(&self) -> Arc<dyn service::auth::AuthPacker + Send + Sync>;
}

pub trait AuthExtractorProvider {
    fn auth_extractor(&self) -> Arc<dyn service::auth::AuthExtractor + Send + Sync>;
}

#[cfg(test)]
pub mod mock {
    use std::sync::Arc;

    use super::{
        database::{mock::MockDatabase, Database},
        service::{
            auth::{mock::MockAuthPacker, AuthPacker},
            email::{mock::MockEmailVerificationService, EmailVerificationService},
        },
        AuthPackerProvider, DatabaseProvider, EmailVerificationServiceProvider,
    };

    pub struct MockDependencyProvider {
        pub db: Arc<MockDatabase>,
        pub email_verification_service: Arc<MockEmailVerificationService>,
        pub auth_packer: Arc<MockAuthPacker>,
    }
    impl MockDependencyProvider {
        pub fn new(db: Arc<MockDatabase>) -> Self {
            Self {
                db,
                email_verification_service: Arc::new(MockEmailVerificationService::new()),
                auth_packer: Arc::new(MockAuthPacker::new()),
            }
        }
    }
    impl DatabaseProvider for MockDependencyProvider {
        type Transaction = ();
        type Error = ();
        fn database(
            &self,
        ) -> Arc<dyn Database<Transaction = Self::Transaction, Error = Self::Error> + Send + Sync>
        {
            self.db.clone()
        }
    }
    impl EmailVerificationServiceProvider for MockDependencyProvider {
        fn email_verification_service(&self) -> Arc<dyn EmailVerificationService + Send + Sync> {
            self.email_verification_service.clone()
        }
    }
    impl AuthPackerProvider for MockDependencyProvider {
        fn auth_packer(&self) -> Arc<dyn AuthPacker + Send + Sync> {
            self.auth_packer.clone()
        }
    }
}
