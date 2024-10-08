use crate::adapter::{
    model::app::{signup_process, user},
    presenter::Present,
};

#[derive(Default)]
pub struct Presenter;

impl Present<signup_process::initialize::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::initialize::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Created a SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to create a SignupProcess: {err}"),
        }
    }
}

impl Present<signup_process::add_email::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::add_email::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Added Email to SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Add Email to SignupProcess: {err}"),
        }
    }
}

impl Present<signup_process::complete::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::complete::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("SignupProcess Completed -> User Created: {:?}", data.record),
            Err(err) => format!("Unable find SignupProcess: {err}"),
        }
    }
}

impl Present<user::update::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: user::update::Result) -> Self::ViewModel {
        match result {
            Ok(()) => "Updated User".to_string(),
            Err(err) => format!("Unable to update user: {err}"),
        }
    }
}

impl Present<user::get_one::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: user::get_one::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("{:?}", data.user),
            Err(err) => format!("Unable to find user: {err}"),
        }
    }
}

impl Present<user::get_all::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: user::get_all::Result) -> Self::ViewModel {
        match result {
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

impl Present<user::delete::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: user::delete::Result) -> Self::ViewModel {
        match result {
            Ok(_) => "Successfully deleted user".to_string(),
            Err(err) => format!("Unable to delete user {err}"),
        }
    }
}
