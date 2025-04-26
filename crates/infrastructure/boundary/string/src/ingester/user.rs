use std::str::FromStr;

use uuid::Uuid;

use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{AuthPackerProvider, DatabaseProvider},
    usecase::user::{
        delete::{Delete, Request as DeleteRequest},
        get_all::{GetAll, Request as GetAllRequest},
        get_one::{GetOne, Request as GetOneRequest},
        login::{Login, Request as LoginRequest},
        update::{Request as UpdateRequest, Update},
    },
};
use ca_domain::entity::user::Id;

use super::super::Boundary;
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
#[async_trait::async_trait]
impl<D> Ingester<D, Update<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = (String, String, String, String);
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Update<D>> {
        let (id, email, username, password) = input;
        id.parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| UpdateRequest {
                id: Id::from(uuid),
                email,
                username,
                password,
            })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, GetOne<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = String;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, GetOne<D>> {
        input
            .parse()
            .map_err(|e: <Uuid as FromStr>::Err| Error::ParseInputError(e.to_string()))
            .map(|uuid: Uuid| GetOneRequest { id: Id::from(uuid) })
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, GetAll<D>> for Boundary
where
    D: DatabaseProvider,
{
    type InputModel = ();
    async fn ingest(_: Self::InputModel) -> UsecaseRequestResult<D, GetAll<D>> {
        Ok(GetAllRequest {})
    }
}
#[async_trait::async_trait]
impl<D> Ingester<D, Login<D>> for Boundary
where
    D: DatabaseProvider + AuthPackerProvider,
{
    type InputModel = (String, String);
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Login<D>> {
        let (username, password) = input;
        Ok(LoginRequest { username, password })
    }
}
