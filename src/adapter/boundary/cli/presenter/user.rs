use crate::{
    adapter::boundary::{
        cli::presenter::Presenter, Presenter as PresenterTrait, UsecaseResponseResult,
    },
    adapter::model::cli::view::ViewModel,
    application::{
        gateway::repository::user::Repo,
        usecase::user::{delete::Delete, get_all::GetAll, get_one::GetOne, update::Update},
    },
};

impl<'d, D> PresenterTrait<'d, D, Update<'d, D>> for Presenter
where
    D: Repo,
{
    type ViewModel = ViewModel;

    fn present(data: UsecaseResponseResult<'d, D, Update<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(()) => "Updated ".to_string(),
            Err(err) => format!("Unable to update user: {err}"),
        }
    }
}

impl<'d, D> PresenterTrait<'d, D, GetOne<'d, D>> for Presenter
where
    D: Repo,
{
    type ViewModel = ViewModel;

    fn present(data: UsecaseResponseResult<'d, D, GetOne<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(data) => format!("{:?}", data.user),
            Err(err) => format!("Unable to find user: {err}"),
        }
    }
}

impl<'d, D> PresenterTrait<'d, D, GetAll<'d, D>> for Presenter
where
    D: Repo,
{
    type ViewModel = ViewModel;

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

impl<'d, D> PresenterTrait<'d, D, Delete<'d, D>> for Presenter
where
    D: Repo,
{
    type ViewModel = ViewModel;

    fn present(data: UsecaseResponseResult<'d, D, Delete<'d, D>>) -> Self::ViewModel {
        match data {
            Ok(_) => "Successfully deleted user".to_string(),
            Err(err) => format!("Unable to delete user {err}"),
        }
    }
}
