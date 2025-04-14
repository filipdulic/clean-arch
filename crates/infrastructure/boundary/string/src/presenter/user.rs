use super::super::Boundary;

use ca_adapter::boundary::{Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{AuthPackerProvider, DatabaseProvider},
    usecase::user::{
        delete::Delete, get_all::GetAll, get_one::GetOne, login::Login, update::Update,
    },
};

impl<'d, D> Presenter<'d, D, Update<'d, D>> for Boundary
where
    D: DatabaseProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Update<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(()) => "Updated ".to_string(),
            Err(err) => format!("Unable to update user: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, GetOne<'d, D>> for Boundary
where
    D: DatabaseProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, GetOne<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("{:?}", data.user),
            Err(err) => format!("Unable to find user: {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, GetAll<'d, D>> for Boundary
where
    D: DatabaseProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, GetAll<'d, D>>) -> Self::ViewModel {
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

impl<'d, D> Presenter<'d, D, Delete<'d, D>> for Boundary
where
    D: DatabaseProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Delete<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(_) => "Successfully deleted user".to_string(),
            Err(err) => format!("Unable to delete user {err}"),
        }
    }
}

impl<'d, D> Presenter<'d, D, Login<'d, D>> for Boundary
where
    D: DatabaseProvider + AuthPackerProvider,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<'d, D, Login<'d, D>>) -> Self::ViewModel {
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
