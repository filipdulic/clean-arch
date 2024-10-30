use crate::{
    application::{
        gateway::repository::{signup_process::Repo as SignupProcessRepo, user::Repo as UserRepo},
        identifier::NewId,
    },
    domain::entity::{signup_process::Id as SignupProcessId, user::Id as UserId},
};

pub trait Db:
    SignupProcessRepo + NewId<SignupProcessId> + UserRepo + NewId<UserId> + 'static + Clone
{
}
