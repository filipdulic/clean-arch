use uuid::Uuid;

use super::super::Boundary;
use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{
        EmailVerificationServiceProvider, SignupProcessIdGenProvider, SignupProcessRepoProvider,
        TokenRepoProvider, UserRepoProvider,
    },
    usecase::signup_process::{
        complete::{Complete, Request as CompleteRequest},
        delete::{Delete, Request as DeleteRequest},
        extend_completion_time::{ExtendCompletionTime, Request as ExtendCompletionTimeRequest},
        extend_verification_time::{
            ExtendVerificationTime, Request as ExtendVerificationTimeRequest,
        },
        get_state_chain::{GetStateChain, Request as GetStateChainRequest},
        initialize::{Initialize, Request as InitializeRequest},
        send_verification_email::{Request as SendVerificationEmailRequest, SendVerificationEmail},
        verify_email::{Request as VerifyEmailRequest, VerifyEmail},
    },
};
use ca_domain::entity::signup_process::Id;

impl<'d, D> Ingester<'d, D, Complete<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + UserRepoProvider,
{
    type InputModel = (String, String, String);
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, Complete<'d, D>> {
        let (id, username, password) = input;
        id.parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| CompleteRequest {
                id: Id::from(uuid),
                username,
                password,
            })
    }
}

impl<'d, D> Ingester<'d, D, ExtendCompletionTime<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, ExtendCompletionTime<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| ExtendCompletionTimeRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, ExtendVerificationTime<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + TokenRepoProvider,
{
    type InputModel = String;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, ExtendVerificationTime<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| ExtendVerificationTimeRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, GetStateChain<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, GetStateChain<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| GetStateChainRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, Initialize<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + SignupProcessIdGenProvider,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, Initialize<'d, D>> {
        Ok(InitializeRequest { email: input })
    }
}

impl<'d, D> Ingester<'d, D, SendVerificationEmail<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + EmailVerificationServiceProvider + TokenRepoProvider,
{
    type InputModel = String;
    fn ingest(
        input: Self::InputModel,
    ) -> UsecaseRequestResult<'d, D, SendVerificationEmail<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| SendVerificationEmailRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, VerifyEmail<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider + TokenRepoProvider,
{
    type InputModel = (String, String);
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, VerifyEmail<'d, D>> {
        let (id, token) = input;
        id.parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| VerifyEmailRequest {
                id: Id::from(uuid),
                token,
            })
    }
}

impl<'d, D> Ingester<'d, D, Delete<'d, D>> for Boundary
where
    D: SignupProcessRepoProvider,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, Delete<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| DeleteRequest { id: Id::from(uuid) })
    }
}
