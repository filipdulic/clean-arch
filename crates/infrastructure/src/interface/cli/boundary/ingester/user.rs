use uuid::Uuid;

use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::repository::user::Repo,
    usecase::user::{
        delete::{Delete, Request as DeleteRequest},
        get_all::{GetAll, Request as GetAllRequest},
        get_one::{GetOne, Request as GetOneRequest},
        update::{Request as UpdateRequest, Update},
    },
};
use ca_domain::entity::user::Id;

use super::super::Boundary;

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

impl<'d, D> Ingester<'d, D, Update<'d, D>> for Boundary
where
    D: Repo,
{
    type InputModel = (String, String, String, String);
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, Update<'d, D>> {
        let (id, email, username, password) = input;
        id.parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| UpdateRequest {
                id: Id::from(uuid),
                email,
                username,
                password,
            })
    }
}

impl<'d, D> Ingester<'d, D, GetOne<'d, D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, GetOne<'d, D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| GetOneRequest { id: Id::from(uuid) })
    }
}

impl<'d, D> Ingester<'d, D, GetAll<'d, D>> for Boundary
where
    D: Repo,
{
    type InputModel = ();
    fn ingest(_: Self::InputModel) -> UsecaseRequestResult<'d, D, GetAll<'d, D>> {
        Ok(GetAllRequest {})
    }
}
