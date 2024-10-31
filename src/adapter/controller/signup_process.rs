use crate::{
    adapter::boundary::{Ingester, Presenter},
    application::{
        gateway::repository as repo,
        identifier::NewId,
        usecase::signup_process::{
            complete::Complete, completion_timed_out::CompletionTimedOut,
            extend_completion_time::ExtendCompletionTime,
            extend_verification_time::ExtendVerificationTime, get_state_chain::GetStateChain,
            initialize::Initialize, verification_timed_out::VerificationTimedOut,
            verify_email::VerifyEmail,
        },
    },
    domain::entity::signup_process,
};

use super::Controller;

pub struct SignupProcessController<'d, D, B> {
    db: &'d D,
    #[allow(dead_code)]
    boundry: B,
}

impl<'d, D, B> Controller<'d, D, B> for SignupProcessController<'d, D, B>
where
    D: repo::signup_process::Repo + repo::user::Repo + NewId<signup_process::Id> + 'd,
    B: Presenter<'d, D, Complete<'d, D>>
        + Presenter<'d, D, CompletionTimedOut<'d, D>>
        + Presenter<'d, D, ExtendCompletionTime<'d, D>>
        + Presenter<'d, D, ExtendVerificationTime<'d, D>>
        + Presenter<'d, D, GetStateChain<'d, D>>
        + Presenter<'d, D, Initialize<'d, D>>
        + Presenter<'d, D, VerificationTimedOut<'d, D>>
        + Presenter<'d, D, VerifyEmail<'d, D>>
        + Ingester<'d, D, Complete<'d, D>>
        + Ingester<'d, D, CompletionTimedOut<'d, D>>
        + Ingester<'d, D, ExtendCompletionTime<'d, D>>
        + Ingester<'d, D, ExtendVerificationTime<'d, D>>
        + Ingester<'d, D, GetStateChain<'d, D>>
        + Ingester<'d, D, Initialize<'d, D>>
        + Ingester<'d, D, VerificationTimedOut<'d, D>>
        + Ingester<'d, D, VerifyEmail<'d, D>>,
{
    fn new(db: &'d D, boundry: B) -> Self {
        Self { db, boundry }
    }

    fn db(&self) -> &'d D {
        self.db
    }
}
