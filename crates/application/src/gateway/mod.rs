use repository::Database;

pub mod repository;
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
