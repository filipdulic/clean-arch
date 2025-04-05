use serde::Deserialize;
use uuid::Uuid;

use super::super::Boundary;
use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{
        EmailVerificationServiceProvider, SignupProcessIdGenProvider, SignupProcessRepoProvider,
        TokenRepoProvider, UserRepoProvider,
    },
    usecase::signup_process as uc,
};
use ca_domain::entity::signup_process::Id;

#[derive(Deserialize)]
pub struct CompleteRequest {
    id: String,
    username: String,
    password: String,
}

impl<'d, D> Ingester<'d, D, uc::complete::Complete<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + UserRepoProvider,
{
    type InputModel = CompleteRequest;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, uc::complete::Complete<'d, D>> {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::complete::Request {
                id: Id::from(uuid),
                username: input.username,
                password: input.password,
            })
    }
}

#[derive(Deserialize)]
pub struct IdRequest {
    id: String,
}

impl<'d, D> Ingester<'d, D, uc::extend_completion_time::ExtendCompletionTime<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider,
{
    type InputModel = IdRequest;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, uc::extend_completion_time::ExtendCompletionTime<'d, D>> {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::extend_completion_time::Request { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, uc::extend_verification_time::ExtendVerificationTime<'d, D>>
    for Boundary
where
    D: SignupProcessRepoProvider + TokenRepoProvider,
{
    type InputModel = IdRequest;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, uc::extend_verification_time::ExtendVerificationTime<'d, D>>
    {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::extend_verification_time::Request { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, uc::get_state_chain::GetStateChain<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider,
{
    type InputModel = IdRequest;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, uc::get_state_chain::GetStateChain<'d, D>> {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::get_state_chain::Request { id: Id::from(uuid) })
    }
}

#[derive(Deserialize)]
pub struct InitializeRequest {
    email: String,
}

impl<'d, D> Ingester<'d, D, uc::initialize::Initialize<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + SignupProcessIdGenProvider,
{
    type InputModel = InitializeRequest;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, uc::initialize::Initialize<'d, D>> {
        Ok(uc::initialize::Request { email: input.email })
    }
}

impl<'d, D> Ingester<'d, D, uc::send_verification_email::SendVerificationEmail<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + EmailVerificationServiceProvider + TokenRepoProvider,
{
    type InputModel = IdRequest;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, uc::send_verification_email::SendVerificationEmail<'d, D>>
    {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::send_verification_email::Request { id: Id::from(uuid) })
    }
}

#[derive(Deserialize)]
pub struct VerifyEmailRequest {
    id: String,
    token: String,
}

impl<'d, D> Ingester<'d, D, uc::verify_email::VerifyEmail<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + TokenRepoProvider,
{
    type InputModel = VerifyEmailRequest;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, uc::verify_email::VerifyEmail<'d, D>> {
        input
            .id
            .parse::<Uuid>()
            .map_err(|_| Error::ParseIdError)
            .map(|uuid: Uuid| uc::verify_email::Request {
                id: Id::from(uuid),
                token: input.token,
            })
    }
}

impl<'d, D> Ingester<'d, D, uc::delete::Delete<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider,
{
    type InputModel = IdRequest;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, uc::delete::Delete<'d, D>> {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::delete::Request { id: Id::from(uuid) })
    }
}
