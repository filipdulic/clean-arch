use crate::{
    adapter::boundary::{string::Boundary, Presenter, UsecaseResponseResult},
    application::{
        gateway::repository::user::Repo,
        usecase::user::{delete::Delete, get_all::GetAll, get_one::GetOne, update::Update},
    },
};

impl<D> Presenter<D, Update<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, Update<D>>) -> Self::ViewModel {
        match data {
            Ok(()) => "Updated ".to_string(),
            Err(err) => format!("Unable to update user: {err}"),
        }
    }
}

impl<D> Presenter<D, GetOne<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, GetOne<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("{:?}", data.user),
            Err(err) => format!("Unable to find user: {err}"),
        }
    }
}

impl<D> Presenter<D, GetAll<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, GetAll<D>>) -> Self::ViewModel {
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

impl<D> Presenter<D, Delete<D>> for Boundary
where
    D: Repo,
{
    type ViewModel = String;

    fn present(data: UsecaseResponseResult<D, Delete<D>>) -> Self::ViewModel {
        match data {
            Ok(_) => "Successfully deleted user".to_string(),
            Err(err) => format!("Unable to delete user {err}"),
        }
    }
}
