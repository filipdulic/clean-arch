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

impl<D> Ingester<D, Complete<D>> for Boundary
where
    D: Repo + UserRepo,
{
    type InputModel = (String, String, String);
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Complete<D>> {
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

impl<D> Ingester<D, CompletionTimedOut<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, CompletionTimedOut<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| CompletionTimedOutRequest { id: Id::from(uuid) })
    }
}

impl<D> Ingester<D, ExtendCompletionTime<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, ExtendCompletionTime<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| ExtendCompletionTimeRequest { id: Id::from(uuid) })
    }
}

impl<D> Ingester<D, ExtendVerificationTime<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, ExtendVerificationTime<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| ExtendVerificationTimeRequest { id: Id::from(uuid) })
    }
}

impl<D> Ingester<D, GetStateChain<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, GetStateChain<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| GetStateChainRequest { id: Id::from(uuid) })
    }
}

impl<D> Ingester<D, Initialize<D>> for Boundary
where
    D: Repo + NewId<Id>,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Initialize<D>> {
        Ok(InitializeRequest { email: input })
    }
}

impl<D> Ingester<D, VerificationTimedOut<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, VerificationTimedOut<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| VerificationTimedOutRequest { id: Id::from(uuid) })
    }
}

impl<D> Ingester<D, VerifyEmail<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, VerifyEmail<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| VerifyEmailRequest { id: Id::from(uuid) })
    }
}

impl<D> Ingester<D, Delete<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Delete<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| DeleteRequest { id: Id::from(uuid) })
    }
}
