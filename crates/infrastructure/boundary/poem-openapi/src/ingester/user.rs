// ========================================
// Delete Use Case
// ========================================

use std::str::FromStr;

use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{AuthPackerProvider, DatabaseProvider},
    usecase::user::{
        delete::{Delete, Request as UsecaseDeleteRequest},
        get_all::{GetAll, Request as UsecaseGetAllRequest},
        get_one::{GetOne, Request as UsecaseGetOneRequest},
        login::{Login, Request as UsecaseLoginRequest},
        update::{Request as UsecaseUpdateRequest, Update},
    },
};
use ca_domain::entity::user::Id;
use poem_openapi::Object;
use uuid::Uuid;

use crate::Boundary;

use super::signup_process::IdRequest;

// ========================================
// Delete Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, Delete<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send,
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
// Get All Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, GetAll<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = ();
    async fn ingest(_: Self::InputModel) -> UsecaseRequestResult<D, GetAll<D>> {
        Ok(UsecaseGetAllRequest)
    }
}

// ========================================
// Get One Use Case
// ========================================

#[async_trait::async_trait]
impl<D> Ingester<D, GetOne<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = IdRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, GetOne<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseGetOneRequest { id: Id::from(uuid) })
    }
}

// ========================================
// Login Use Case
// ========================================

#[derive(Object)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[async_trait::async_trait]
impl<D> Ingester<D, Login<D>> for Boundary
where
    D: DatabaseProvider + AuthPackerProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = LoginRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Login<D>> {
        Ok(UsecaseLoginRequest {
            username: input.username,
            password: input.password,
        })
    }
}

// ========================================
// Upadte Use Case
// ========================================

#[derive(Object)]
pub struct UpdateRequest {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[async_trait::async_trait]
impl<D> Ingester<D, Update<D>> for Boundary
where
    D: DatabaseProvider + AuthPackerProvider + std::marker::Sync + std::marker::Send,
{
    type InputModel = UpdateRequest;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Update<D>> {
        input
            .id
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UsecaseUpdateRequest {
                id: Id::from(uuid),
                username: input.username,
                email: input.email,
                password: input.password,
            })
    }
}
