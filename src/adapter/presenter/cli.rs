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

impl Present<signup_process::verify_email::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::verify_email::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Email Verified of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl Present<signup_process::verification_timed_out::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::verification_timed_out::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Verification Timed Out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl Present<signup_process::completion_timed_out::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::completion_timed_out::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("Completion Timed Out of SignupProcess(ID = {})", data.id),
            Err(err) => format!("Unable to Verify Email of SignupProcess: {err}"),
        }
    }
}

impl Present<signup_process::delete::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::delete::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("SignupProcess(ID = {}) scheduled for deletion", data.id),
            Err(err) => format!("Unable to delete SignupProcess: {err}"),
        }
    }
}

impl Present<signup_process::extend_verification_time::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::extend_verification_time::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!(
                "Verification time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend verification time of SignupProcess: {err}"),
        }
    }
}

impl Present<signup_process::extend_completion_time::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::extend_completion_time::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!(
                "Completion time extended of SignupProcess(ID = {})",
                data.id
            ),
            Err(err) => format!("Unable to extend completion time of SignupProcess: {err}"),
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

impl Present<signup_process::get_state_chain::Result> for Presenter {
    type ViewModel = String;
    fn present(&self, result: signup_process::get_state_chain::Result) -> Self::ViewModel {
        match result {
            Ok(data) => format!("{:?}", data.state_chain),
            Err(err) => format!("Unable to get state chain: {err}"),
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
