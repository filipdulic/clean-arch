use crate::{
    adapter::presenter::Present,
    application::usecase::{signup_process as sp, user},
};

#[derive(Default)]
pub struct Presenter;

impl Present<Result<sp::initialize::Response, sp::initialize::Error>> for Presenter {
    type ViewModel = String;
    fn present(&self, result: Result<sp::initialize::Response, sp::initialize::Error>) -> String {
        match result {
            Ok(data) => format!("Created a SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to create a SignupProcess: {err}"),
        }
    }
}

impl Present<Result<sp::verify_email::Response, sp::verify_email::Error>> for Presenter {
    type ViewModel = String;
    fn present(
        &self,
        result: Result<sp::verify_email::Response, sp::verify_email::Error>,
    ) -> String {
        match result {
            Ok(data) => format!("Email Verified of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl Present<Result<sp::verification_timed_out::Response, sp::verification_timed_out::Error>>
    for Presenter
{
    type ViewModel = String;
    fn present(
        &self,
        result: Result<sp::verification_timed_out::Response, sp::verification_timed_out::Error>,
    ) -> String {
        match result {
            Ok(data) => format!("Verification timed out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to time out Verification of SignupProcess: {err}"),
        }
    }
}

impl Present<Result<sp::extend_verification_time::Response, sp::extend_verification_time::Error>>
    for Presenter
{
    type ViewModel = String;
    fn present(
        &self,
        result: Result<sp::extend_verification_time::Response, sp::extend_verification_time::Error>,
    ) -> String {
        match result {
            Ok(data) => format!(
                "Verification time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend Verification time of SignupProcess: {err}"),
        }
    }
}

impl Present<Result<sp::completion_timed_out::Response, sp::completion_timed_out::Error>>
    for Presenter
{
    type ViewModel = String;
    fn present(
        &self,
        result: Result<sp::completion_timed_out::Response, sp::completion_timed_out::Error>,
    ) -> String {
        match result {
            Ok(data) => format!("Completion timed out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to time out Completion of SignupProcess: {err}"),
        }
    }
}

impl Present<Result<sp::extend_completion_time::Response, sp::extend_completion_time::Error>>
    for Presenter
{
    type ViewModel = String;
    fn present(
        &self,
        result: Result<sp::extend_completion_time::Response, sp::extend_completion_time::Error>,
    ) -> String {
        match result {
            Ok(data) => format!(
                "Completion time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend Completion time of SignupProcess: {err}"),
        }
    }
}

impl Present<Result<sp::complete::Response, sp::complete::Error>> for Presenter {
    type ViewModel = String;
    fn present(&self, result: Result<sp::complete::Response, sp::complete::Error>) -> String {
        match result {
            Ok(data) => format!("SignupProcess Completed -> User Created: {:?}", data.record),
            Err(err) => format!("Unable to complete SignupProcess: {err}"),
        }
    }
}

impl Present<Result<user::update::Response, user::update::Error>> for Presenter {
    type ViewModel = String;
    fn present(&self, result: Result<user::update::Response, user::update::Error>) -> String {
        match result {
            Ok(()) => "Updated User".to_string(),
            Err(err) => format!("Unable to update user: {err}"),
        }
    }
}

impl Present<Result<user::get_one::Response, user::get_one::Error>> for Presenter {
    type ViewModel = String;
    fn present(&self, result: Result<user::get_one::Response, user::get_one::Error>) -> String {
        match result {
            Ok(data) => format!("{:?}", data.user),
            Err(err) => format!("Unable to find user: {err}"),
        }
    }
}

impl Present<Result<user::get_all::Response, user::get_all::Error>> for Presenter {
    type ViewModel = String;
    fn present(&self, result: Result<user::get_all::Response, user::get_all::Error>) -> String {
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

impl Present<Result<user::delete::Response, user::delete::Error>> for Presenter {
    type ViewModel = String;
    fn present(&self, result: Result<user::delete::Response, user::delete::Error>) -> String {
        match result {
            Ok(_) => "Successfully deleted user".to_string(),
            Err(err) => format!("Unable to delete user: {err}"),
        }
    }
}
