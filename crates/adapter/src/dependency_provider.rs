use ca_application::gateway::{
    EmailVerificationServiceProvider, SignupProcessIdGenProvider, SignupProcessRepoProvider,
    TokenRepoProvider, UserIdGenProvider, UserRepoProvider,
};

pub trait Transactional:
    Clone
    + SignupProcessRepoProvider
    + SignupProcessIdGenProvider
    + UserRepoProvider
    + UserIdGenProvider
    + EmailVerificationServiceProvider
    + TokenRepoProvider
{
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> Result<R, E>
    where
        F: FnOnce(&'d Self) -> Result<R, E>;
}
