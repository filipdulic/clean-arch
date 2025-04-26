use ca_adapter::boundary::{Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
    usecase::signup_process::{
        complete::Complete, delete::Delete, extend_completion_time::ExtendCompletionTime,
        extend_verification_time::ExtendVerificationTime, get_state_chain::GetStateChain,
        initialize::Initialize, send_verification_email::SendVerificationEmail,
        verify_email::VerifyEmail,
    },
};

use super::super::Boundary;
#[async_trait::async_trait]
impl<D> Presenter<D, Complete<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, Complete<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("SignupProcess Completed -> User Created: {:?}", data.record),
            Err(err) => format!("Unable find SignupProcess: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, Delete<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, Delete<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("SignupProcess(ID = {}) scheduled for deletion", data.id),
            Err(err) => format!("Unable to delete SignupProcess: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, ExtendCompletionTime<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, ExtendCompletionTime<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!(
                "Completion time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend completion time of SignupProcess: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, ExtendVerificationTime<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, ExtendVerificationTime<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!(
                "Verification time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend verification time of SignupProcess: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, GetStateChain<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, GetStateChain<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("{:?}", data.state_chain),
            Err(err) => format!("Unable to get state chain: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, Initialize<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, Initialize<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Created a SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to create a SignupProcess: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, SendVerificationEmail<D>> for Boundary
where
    D: DatabaseProvider + 'static + EmailVerificationServiceProvider,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, SendVerificationEmail<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Verification email sent(ID = {})", data.id),
            Err(err) => format!("Unable to send verification email: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, VerifyEmail<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, VerifyEmail<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Email Verified of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}
