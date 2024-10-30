use std::sync::Arc;

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

pub struct SignupProcessController<D, B> {
    db: Arc<D>,
    #[allow(dead_code)]
    boundry: B,
}

impl<D, B> Controller<D, B> for SignupProcessController<D, B>
where
    D: repo::signup_process::Repo + repo::user::Repo + NewId<signup_process::Id>,
    B: Presenter<D, Complete<D>>
        + Presenter<D, CompletionTimedOut<D>>
        + Presenter<D, ExtendCompletionTime<D>>
        + Presenter<D, ExtendVerificationTime<D>>
        + Presenter<D, GetStateChain<D>>
        + Presenter<D, Initialize<D>>
        + Presenter<D, VerificationTimedOut<D>>
        + Presenter<D, VerifyEmail<D>>
        + Ingester<D, Complete<D>>
        + Ingester<D, CompletionTimedOut<D>>
        + Ingester<D, ExtendCompletionTime<D>>
        + Ingester<D, ExtendVerificationTime<D>>
        + Ingester<D, GetStateChain<D>>
        + Ingester<D, Initialize<D>>
        + Ingester<D, VerificationTimedOut<D>>
        + Ingester<D, VerifyEmail<D>>,
{
    fn new(db: Arc<D>, boundry: B) -> Self {
        Self { db, boundry }
    }

    fn db(&self) -> Arc<D> {
        self.db.clone()
    }
}
