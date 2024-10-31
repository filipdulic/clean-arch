use uuid::Uuid;

use crate::{
    adapter::boundary::{string::Boundary, Error, Ingester, UsecaseRequestResult},
    application::{
        gateway::repository::{signup_process::Repo, user::Repo as UserRepo},
        identifier::NewId,
        usecase::signup_process::{
            complete::{Complete, Request as CompleteRequest},
            completion_timed_out::{CompletionTimedOut, Request as CompletionTimedOutRequest},
            delete::{Delete, Request as DeleteRequest},
            extend_completion_time::{
                ExtendCompletionTime, Request as ExtendCompletionTimeRequest,
            },
            extend_verification_time::{
                ExtendVerificationTime, Request as ExtendVerificationTimeRequest,
            },
            get_state_chain::{GetStateChain, Request as GetStateChainRequest},
            initialize::{Initialize, Request as InitializeRequest},
            verification_timed_out::{
                Request as VerificationTimedOutRequest, VerificationTimedOut,
            },
            verify_email::{Request as VerifyEmailRequest, VerifyEmail},
        },
    },
    domain::entity::signup_process::Id,
};

impl<'d, D> Ingester<'d, D, Complete<'d, D>> for Boundary
where
    D: Repo + UserRepo,
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

impl<'d, D> Ingester<'d, D, CompletionTimedOut<'d, D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, CompletionTimedOut<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| CompletionTimedOutRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, ExtendCompletionTime<'d, D>> for Boundary
where
    D: Repo,
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
    D: Repo,
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
    D: Repo,
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
    D: Repo + NewId<Id>,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, Initialize<'d, D>> {
        Ok(InitializeRequest { email: input })
    }
}

impl<'d, D> Ingester<'d, D, VerificationTimedOut<'d, D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, VerificationTimedOut<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| VerificationTimedOutRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, VerifyEmail<'d, D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, VerifyEmail<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| VerifyEmailRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, Delete<'d, D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, Delete<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| DeleteRequest { id: Id::from(uuid) })
    }
}
