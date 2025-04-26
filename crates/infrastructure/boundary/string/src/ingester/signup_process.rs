use std::str::FromStr;

use uuid::Uuid;

use super::super::Boundary;
use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
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
#[async_trait::async_trait]
impl<D> Ingester<D, Complete<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = (String, String, String);
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Complete<D>> {
        let (id, username, password) = input;
        id.parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| CompleteRequest {
                id: Id::from(uuid),
                username,
                password,
            })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, ExtendCompletionTime<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = String;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, ExtendCompletionTime<D>> {
        input
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| ExtendCompletionTimeRequest { id: Id::from(uuid) })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, ExtendVerificationTime<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = String;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, ExtendVerificationTime<D>> {
        input
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| ExtendVerificationTimeRequest { id: Id::from(uuid) })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, GetStateChain<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = String;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, GetStateChain<D>> {
        input
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| GetStateChainRequest { id: Id::from(uuid) })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, Initialize<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = String;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Initialize<D>> {
        Ok(InitializeRequest { email: input })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, SendVerificationEmail<D>> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type InputModel = String;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, SendVerificationEmail<D>> {
        input
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| SendVerificationEmailRequest { id: Id::from(uuid) })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, VerifyEmail<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = (String, String);
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, VerifyEmail<D>> {
        let (id, token) = input;
        id.parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| VerifyEmailRequest {
                id: Id::from(uuid),
                token,
            })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, Delete<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = String;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Delete<D>> {
        input
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| DeleteRequest { id: Id::from(uuid) })
    }
}
