pub mod signup_process {
    pub use crate::json_boundary::{
        domain::{SignupProcess, SignupProcessId},
        usecase::signup_process::*,
    };
}
pub mod user {
    pub use crate::json_boundary::{
        domain::{User, UserId},
        usecase::user::*,
    };
}
pub use crate::json_boundary::{Error, Response, Result, StatusCode};
