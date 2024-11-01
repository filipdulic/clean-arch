use crate::{
    application::{
        gateway::repository::{signup_process::Repo as SignupProcessRepo, user::Repo as UserRepo},
        identifier::NewId,
    },
    domain::entity::{signup_process::Id as SignupProcessId, user::Id as UserId},
};

pub trait Db:
    SignupProcessRepo + NewId<SignupProcessId> + UserRepo + NewId<UserId> + Clone
{
}

pub trait Transactional {
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> Result<R, E>
    where
        F: FnOnce(&'d Self) -> Result<R, E>;
}
