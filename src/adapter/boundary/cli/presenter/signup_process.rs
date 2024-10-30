use crate::{
    adapter::boundary::{cli::Boundary, Presenter, UsecaseResponseResult},
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

impl<D> Presenter<D, Complete<D>> for Boundary
where
    D: Repo + UserRepo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, Complete<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("SignupProcess Completed -> User Created: {:?}", data.record),
            Err(err) => format!("Unable find SignupProcess: {err}"),
        }
    }
}

impl<D> Presenter<D, CompletionTimedOut<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, CompletionTimedOut<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Completion Timed Out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl<D> Presenter<D, Delete<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, Delete<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("SignupProcess(ID = {}) scheduled for deletion", data.id),
            Err(err) => format!("Unable to delete SignupProcess: {err}"),
        }
    }
}

impl<D> Presenter<D, ExtendCompletionTime<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, ExtendCompletionTime<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!(
                "Completion time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend completion time of SignupProcess: {err}"),
        }
    }
}

impl<D> Presenter<D, ExtendVerificationTime<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, ExtendVerificationTime<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!(
                "Verification time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend verification time of SignupProcess: {err}"),
        }
    }
}

impl<D> Presenter<D, GetStateChain<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, GetStateChain<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("{:?}", data.state_chain),
            Err(err) => format!("Unable to get state chain: {err}"),
        }
    }
}

impl<D> Presenter<D, Initialize<D>> for Boundary
where
    D: Repo + NewId<Id>,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, Initialize<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Created a SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to create a SignupProcess: {err}"),
        }
    }
}

impl<D> Presenter<D, VerificationTimedOut<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, VerificationTimedOut<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Verification Timed Out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl<D> Presenter<D, VerifyEmail<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, VerifyEmail<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Email Verified of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}
