use std::str::FromStr;

use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::DatabaseProvider,
    usecase::signup_process::{
        complete::{Complete, Request as UsecaseCompleteRequest},
        initialize::{Initialize, Request as UsecaseInitializeRequest},
    },
};
use ca_domain::entity::signup_process::Id;
use poem_openapi::Object;
use uuid::Uuid;

use crate::Boundary;

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
