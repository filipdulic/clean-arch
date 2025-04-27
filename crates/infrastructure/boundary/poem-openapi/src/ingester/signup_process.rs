use std::str::FromStr;

use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
    usecase::signup_process::{
        complete::{Complete, Request as UsecaseCompleteRequest},
        delete::{Delete, Request as UsecaseDeleteRequest},
        extend_completion_time::{
            ExtendCompletionTime, Request as UsecaseExtendCompletionTimeRequest,
        },
        extend_verification_time::{
            ExtendVerificationTime, Request as UsecaseExtendVerificationTimeRequest,
        },
        get_state_chain::{GetStateChain, Request as UsecaseGetStateChainRequest},
        initialize::{Initialize, Request as UsecaseInitializeRequest},
        send_verification_email::{
            Request as UsecaseSendVerificationEmailRequest, SendVerificationEmail,
        },
        verify_email::{Request as UsecaseVerifyEmailRequest, VerifyEmail},
    },
};
use ca_domain::entity::signup_process::Id;
use poem_openapi::Object;
use uuid::Uuid;

use crate::Boundary;

#[derive(Object)]
pub struct IdRequest {
    pub id: String,
}

// ========================================
// Complete Use Case
// ========================================

#[derive(Object)]
pub struct CompleteRequest {
    pub id: String,
    pub username: String,
    pub password: String,
}
#[async_trait::async_trait]
impl<D> Ingester<D, Complete<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = CompleteRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Complete<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseCompleteRequest {
                id: Id::from(uuid),
                username: input.username,
                password: input.password,
            })
    }
}

// ========================================
// Delete Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, Delete<D>> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = IdRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Delete<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseDeleteRequest { id: Id::from(uuid) })
    }
}

// ========================================
// Extend Completion Time Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, ExtendCompletionTime<D>> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = IdRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, ExtendCompletionTime<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseExtendCompletionTimeRequest { id: Id::from(uuid) })
    }
}

// ========================================
// Extend Verification Time Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, ExtendVerificationTime<D>> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = IdRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, ExtendVerificationTime<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseExtendVerificationTimeRequest { id: Id::from(uuid) })
    }
}

// ========================================
// Get State Chain Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, GetStateChain<D>> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = IdRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, GetStateChain<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseGetStateChainRequest { id: Id::from(uuid) })
    }
}

// ========================================
// Initialize Use Case
// ========================================

#[derive(Object)]
pub struct InitializeRequest {
    pub email: String,
}
#[async_trait::async_trait]
impl<D> Ingester<D, Initialize<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = InitializeRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Initialize<D>> {
        Ok(UsecaseInitializeRequest { email: input.email })
    }
}

// ========================================
// Send Verification Email Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, SendVerificationEmail<D>> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = IdRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, SendVerificationEmail<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseSendVerificationEmailRequest { id: Id::from(uuid) })
    }
}

// ========================================
// Verify Email Use Case
// ========================================

#[derive(Object)]
pub struct VerifyEmailRequest {
    pub id: String,
    pub token: String,
}
#[async_trait::async_trait]
impl<D> Ingester<D, VerifyEmail<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = VerifyEmailRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, VerifyEmail<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseVerifyEmailRequest {
                id: Id::from(uuid),
                token: input.token,
            })
    }
}
