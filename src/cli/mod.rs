//! Command line interface. Parses commands, builds usecase requests
//! and dispatches them through the api.
use std::sync::Arc;

use clap::Subcommand;
use uuid::Uuid;

use crate::{
    adapter::{api::Api, db::Db, presenter::cli::Presenter},
    application::usecase::{
        signup_process::{
            complete::{self, Complete},
            completion_timed_out::{self, CompletionTimedOut},
            extend_completion_time::{self, ExtendCompletionTime},
            extend_verification_time::{self, ExtendVerificationTime},
            initialize::{self, Initialize},
            verification_timed_out::{self, VerificationTimedOut},
            verify_email::{self, VerifyEmail},
        },
        user::{
            delete::{self, Delete},
            get_all::{self, GetAll},
            get_one::{self, GetOne},
            update::{self, Update},
        },
    },
    domain::value_object::Id,
};

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Initialize signup process", alias = "sp-init")]
    InitializeSignupProcess { email: String },
    #[clap(about = "Verify email of signup process", alias = "sp-verify")]
    VerifyEmailOfSignupProcess { id: String },
    #[clap(
        about = "Verification of signup process timed out",
        alias = "sp-verify-timeout"
    )]
    VerificationOfSignupProcessTimedOut { id: String },
    #[clap(
        about = "Extend verification time of signup process",
        alias = "sp-extend-verify"
    )]
    ExtendVerificationTimeOfSignupProcess { id: String },
    #[clap(
        about = "Completion of signup process timed out",
        alias = "sp-complete-timeout"
    )]
    CompletionOfSignupProcessTimedOut { id: String },
    #[clap(
        about = "Extend completion time of signup process",
        alias = "sp-extend-complete"
    )]
    ExtendCompletionTimeOfSignupProcess { id: String },
    #[clap(about = "Complete signup process", alias = "sp-complete")]
    CompleteSignupProcess {
        id: String,
        username: String,
        password: String,
    },
    #[clap(about = "List all users")]
    ListUsers,
    #[clap(about = "Read user")]
    ReadUser { id: String },
    #[clap(about = "Update user")]
    UpdateUser {
        id: String,
        email: String,
        username: String,
        password: String,
    },
    #[clap(about = "Delete user")]
    DeleteUser { id: String },
}

pub fn run<D: Db>(db: Arc<D>, cmd: Command) {
    let api = Api::new(db, Presenter);

    let output = match cmd {
        Command::InitializeSignupProcess { email } => {
            api.handle::<Initialize>(initialize::Request { email })
        }
        Command::VerifyEmailOfSignupProcess { id } => match parse_id(&id) {
            Ok(id) => api.handle::<VerifyEmail>(verify_email::Request { id }),
            Err(msg) => msg,
        },
        Command::VerificationOfSignupProcessTimedOut { id } => match parse_id(&id) {
            Ok(id) => api.handle::<VerificationTimedOut>(verification_timed_out::Request { id }),
            Err(msg) => msg,
        },
        Command::ExtendVerificationTimeOfSignupProcess { id } => match parse_id(&id) {
            Ok(id) => {
                api.handle::<ExtendVerificationTime>(extend_verification_time::Request { id })
            }
            Err(msg) => msg,
        },
        Command::CompletionOfSignupProcessTimedOut { id } => match parse_id(&id) {
            Ok(id) => api.handle::<CompletionTimedOut>(completion_timed_out::Request { id }),
            Err(msg) => msg,
        },
        Command::ExtendCompletionTimeOfSignupProcess { id } => match parse_id(&id) {
            Ok(id) => api.handle::<ExtendCompletionTime>(extend_completion_time::Request { id }),
            Err(msg) => msg,
        },
        Command::CompleteSignupProcess {
            id,
            username,
            password,
        } => match parse_id(&id) {
            Ok(id) => api.handle::<Complete>(complete::Request {
                id,
                username,
                password,
            }),
            Err(msg) => msg,
        },
        Command::ListUsers => api.handle::<GetAll>(get_all::Request),
        Command::ReadUser { id } => match parse_id(&id) {
            Ok(id) => api.handle::<GetOne>(get_one::Request { id }),
            Err(msg) => msg,
        },
        Command::UpdateUser {
            id,
            email,
            username,
            password,
        } => match parse_id(&id) {
            Ok(id) => api.handle::<Update>(update::Request {
                id,
                email,
                username,
                password,
            }),
            Err(msg) => msg,
        },
        Command::DeleteUser { id } => match parse_id(&id) {
            Ok(id) => api.handle::<Delete>(delete::Request { id }),
            Err(msg) => msg,
        },
    };
    println!("{output}");
}

fn parse_id<T>(raw: &str) -> Result<Id<T>, String> {
    raw.parse::<Uuid>()
        .map(Id::new)
        .map_err(|_| format!("Unable to parse id: {raw}"))
}
