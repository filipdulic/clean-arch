use crate::{
    adapter::boundary::{string::Boundary, Presenter, UsecaseResponseResult},
    application::{
        gateway::repository::{signup_process::Repo, user::Repo as UserRepo},
        identifier::NewId,
        usecase::signup_process::{
            complete::Complete, completion_timed_out::CompletionTimedOut, delete::Delete,
            extend_completion_time::ExtendCompletionTime,
            extend_verification_time::ExtendVerificationTime, get_state_chain::GetStateChain,
            initialize::Initialize, verification_timed_out::VerificationTimedOut,
            verify_email::VerifyEmail,
        },
    },
    domain::entity::signup_process::Id,
};

impl<'d, D> Presenter<'d, D, Complete<'d, D>> for Boundary
where
    D: Repo + UserRepo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Complete<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("SignupProcess Completed -> User Created: {:?}", data.record),
            Err(err) => format!("Unable find SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, CompletionTimedOut<'d, D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, CompletionTimedOut<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Completion Timed Out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, Delete<'d, D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Delete<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("SignupProcess(ID = {}) scheduled for deletion", data.id),
            Err(err) => format!("Unable to delete SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, ExtendCompletionTime<'d, D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, ExtendCompletionTime<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!(
                "Completion time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend completion time of SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, ExtendVerificationTime<'d, D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(
        data: UsecaseResponseResult<'d, D, ExtendVerificationTime<'d, D>>,
    ) -> Self::ViewModel {
        match data {
            Ok(data) => format!(
                "Verification time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend verification time of SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, GetStateChain<'d, D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, GetStateChain<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("{:?}", data.state_chain),
            Err(err) => format!("Unable to get state chain: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, Initialize<'d, D>> for Boundary
where
    D: Repo + NewId<Id>,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Initialize<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Created a SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to create a SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, VerificationTimedOut<'d, D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, VerificationTimedOut<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Verification Timed Out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, VerifyEmail<'d, D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, VerifyEmail<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Email Verified of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}
