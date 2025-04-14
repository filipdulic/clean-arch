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

impl<'d, D> Presenter<'d, D, Complete<'d, D>> for Boundary
where
    D: DatabaseProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Complete<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("SignupProcess Completed -> User Created: {:?}", data.record),
            Err(err) => format!("Unable find SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, Delete<'d, D>> for Boundary
where
    D: DatabaseProvider,
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
    D: DatabaseProvider,
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
    D: DatabaseProvider,
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
    D: DatabaseProvider,
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
    D: DatabaseProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Initialize<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Created a SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to create a SignupProcess: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, SendVerificationEmail<'d, D>> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type ViewModel = String;

    fn present(
        data: UsecaseResponseResult<'d, D, SendVerificationEmail<'d, D>>,
    ) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Verification email sent(ID = {})", data.id),
            Err(err) => format!("Unable to send verification email: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, VerifyEmail<'d, D>> for Boundary
where
    D: DatabaseProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, VerifyEmail<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("Email Verified of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}
