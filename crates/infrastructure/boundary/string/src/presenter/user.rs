use super::super::Boundary;

use ca_adapter::boundary::{Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{AuthPackerProvider, DatabaseProvider},
    usecase::user::{
        delete::Delete, get_all::GetAll, get_one::GetOne, login::Login, update::Update,
    },
};
#[async_trait::async_trait]
impl<D> Presenter<D, Update<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, Update<D>>) -> Self::ViewModel {
        match data {
            Ok(()) => "Updated ".to_string(),
            Err(err) => format!("Unable to update user: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, GetOne<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, GetOne<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("{:?}", data.user),
            Err(err) => format!("Unable to find user: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, GetAll<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, GetAll<D>>) -> Self::ViewModel {
        match data {
            Ok(resp) => resp
                .users
                .into_iter()
                .map(|t| format!("- {} ({})", t.username(), t.id()))
                .collect::<Vec<_>>()
                .join("\n"),
            Err(err) => format!("Unable to read all users: {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, Delete<D>> for Boundary
where
    D: DatabaseProvider + 'static,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, Delete<D>>) -> Self::ViewModel {
        match data {
            Ok(_) => "Successfully deleted user".to_string(),
            Err(err) => format!("Unable to delete user {err}"),
        }
    }
}
#[async_trait::async_trait]
impl<D> Presenter<D, Login<D>> for Boundary
where
    D: DatabaseProvider + 'static + AuthPackerProvider,
{
    type ViewModel = String;

    async fn present(data: UsecaseResponseResult<D, Login<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!(
                "TOKEN: {:?}\nUSER_ID: {:?}",
                data.token,
                data.user_id.to_string()
            ),
            Err(err) => format!("Unable to find user: {err}"),
        }
    }
}
