use super::super::Boundary;
use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::UserRepoProvider,
    usecase::user::{self as uc},
};
use ca_domain::entity::user::Id;
use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct IdRequest {
    id: String,
}

impl<'d, D> Ingester<'d, D, uc::delete::Delete<'d, D>> for Boundary
where
    D: UserRepoProvider,
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

impl<'d, D> Ingester<'d, D, uc::get_one::GetOne<'d, D>> for Boundary
where
    D: UserRepoProvider,
{
    type InputModel = IdRequest;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, uc::get_one::GetOne<'d, D>> {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::get_one::Request { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, uc::get_all::GetAll<'d, D>> for Boundary
where
    D: UserRepoProvider,
{
    type InputModel = ();
    fn ingest(_: Self::InputModel) -> UsecaseRequestResult<'d, D, uc::get_all::GetAll<'d, D>> {
        Ok(uc::get_all::Request {})
    }
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    id: String,
    username: String,
    email: String,
    password: String,
}

impl<'d, D> Ingester<'d, D, uc::update::Update<'d, D>> for Boundary
where
    D: UserRepoProvider,
{
    type InputModel = UpdateRequest;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, uc::update::Update<'d, D>> {
        input
            .id
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| uc::update::Request {
                id: Id::from(uuid),
                email: input.email,
                username: input.username,
                password: input.password,
            })
    }
}
