use database::Database;

pub mod database;
pub mod service;

pub trait DatabaseProvider {
    fn database(&self) -> impl Database;
}

pub trait EmailVerificationServiceProvider {
    fn email_verification_service(&self) -> impl service::email::EmailVerificationService;
}

pub trait AuthPackerProvider {
    fn auth_packer(&self) -> impl service::auth::AuthPacker;
}

pub trait AuthExtractorProvider {
    fn auth_extractor(&self) -> impl service::auth::AuthExtractor;
}

#[cfg(test)]
pub mod mock {
    use super::{
        database::{mock::MockDatabase, Database},
        service::email::{mock::MockEmailVerificationService, EmailVerificationService},
        DatabaseProvider, EmailVerificationServiceProvider,
    };

    #[derive(Default)]
    pub struct MockDependencyProvider {
        pub db: MockDatabase,
        pub email_verification_service: MockEmailVerificationService,
    }
    impl DatabaseProvider for MockDependencyProvider {
        fn database(&self) -> impl Database {
            &self.db
        }
    }
    impl EmailVerificationServiceProvider for MockDependencyProvider {
        fn email_verification_service(&self) -> impl EmailVerificationService {
            &self.email_verification_service
        }
    }
}
