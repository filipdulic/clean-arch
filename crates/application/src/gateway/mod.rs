pub mod repository;
pub mod service;

pub trait SignupProcessRepoProvider {
    fn signup_process_repo(&self) -> &dyn repository::signup_process::Repo;
}

pub trait UserRepoProvider {
    fn user_repo(&self) -> &dyn repository::user::Repo;
}

pub trait UserIdGenProvider {
    fn user_id_gen(&self) -> &dyn super::identifier::NewId<ca_domain::entity::user::Id>;
}

pub trait SignupProcessIdGenProvider {
    fn signup_process_id_gen(
        &self,
    ) -> &dyn super::identifier::NewId<ca_domain::entity::signup_process::Id>;
}

pub trait EmailVerificationServiceProvider {
    fn email_verification_service(&self) -> &dyn service::email::EmailVerificationService;
}

pub trait TokenRepoProvider {
    fn token_repo(&self) -> &dyn repository::token::Repo;
}

pub trait AuthPackerProvider {
    fn auth_packer(&self) -> &dyn service::auth::AuthPacker;
}

pub trait AuthExtractorProvider {
    fn auth_extractor(&self) -> &dyn service::auth::AuthExtractor;
}
