use ca_application::gateway::{
    SignupProcessIdGenProvider, SignupProcessRepoProvider, UserIdGenProvider, UserRepoProvider,
};

pub trait Transactional:
    Clone
    + SignupProcessRepoProvider
    + SignupProcessIdGenProvider
    + UserRepoProvider
    + UserIdGenProvider
{
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> Result<R, E>
    where
        F: FnOnce(&'d Self) -> Result<R, E>;
}
