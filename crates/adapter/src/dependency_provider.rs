use ca_application::{
    gateway::{
        AuthExtractorProvider, AuthPackerProvider, EmailVerificationServiceProvider,
        SignupProcessIdGenProvider, SignupProcessRepoProvider, TokenRepoProvider,
        UserIdGenProvider, UserRepoProvider,
    },
    usecase::Comitable,
};

pub trait Transactional:
    Clone
    + SignupProcessRepoProvider
    + SignupProcessIdGenProvider
    + UserRepoProvider
    + UserIdGenProvider
    + EmailVerificationServiceProvider
    + TokenRepoProvider
    + AuthPackerProvider
    + AuthExtractorProvider
{
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> Result<R, E>
    where
        Result<R, E>: Into<Comitable<R, E>>,
        F: FnOnce(&'d Self) -> Result<R, E>;
}
