use uuid::Uuid;

use crate::{
    adapter::boundary::{cli::Boundary, Error, Ingester, UsecaseRequestResult},
    application::{
        gateway::repository::user::Repo,
        usecase::user::{
            delete::{Delete, Request as DeleteRequest},
            get_all::{GetAll, Request as GetAllRequest},
            get_one::{GetOne, Request as GetOneRequest},
            update::{Request as UpdateRequest, Update},
        },
    },
    domain::entity::user::Id,
};

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

impl<D> Ingester<D, Update<D>> for Boundary
where
    D: Repo,
{
    type InputModel = (String, String, String, String);
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, Update<D>> {
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

impl<D> Ingester<D, GetOne<D>> for Boundary
where
    D: Repo,
{
    type InputModel = String;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, GetOne<D>> {
        input
            .parse()
            .map_err(|_| Error::ParseInputError)
            .map(|uuid: Uuid| GetOneRequest { id: Id::from(uuid) })
    }
}

impl<D> Ingester<D, GetAll<D>> for Boundary
where
    D: Repo,
{
    type InputModel = ();
    fn ingest(_: Self::InputModel) -> UsecaseRequestResult<D, GetAll<D>> {
        Ok(GetAllRequest {})
    }
}
