//! This module contains the CLI interface for the application.
//!
//! Handles command-line interface (CLI) interactions. It defines the commands
//! that the CLI can execute and maps them to the appropriate actions within
//! the application. This file typically uses a library like clap to parse and
//! handle command-line arguments.
//!
//! Key Responsibilities:
//! * Command Definition: Define the various commands that the CLI can handle.
//! * Command Parsing: Use a library like clap to parse the command-line arguments
//!     and match them to the defined commands.
//! * Command Execution: Map the parsed commands to the appropriate functions or
//!     methods in the application.
use std::sync::Arc;

use clap::Subcommand;

use ca_adapter::{controller::Controller, dependency_provider::Transactional};
use ca_application::usecase::{
    signup_process::{
        complete::Complete, delete::Delete, extend_completion_time::ExtendCompletionTime,
        extend_verification_time::ExtendVerificationTime, get_state_chain::GetStateChain,
        initialize::Initialize, send_verification_email::SendVerificationEmail,
        verify_email::VerifyEmail,
    },
    user::{
        delete::Delete as UserDelete, get_all::GetAll, get_one::GetOne, login::Login,
        update::Update,
    },
};

use ca_infrastructure_boundary_json as boundary;

//use crate::boundary::string::
#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Initialize signup process", alias = "sp-init")]
    InitializeSignupProcess {
        request: String,
        token: Option<String>,
    },
    #[clap(
        about = "Send verification email for signup process",
        alias = "sp-send-verify"
    )]
    SendVerificationEmail {
        request: String,
        token: Option<String>,
    },
    #[clap(
        about = "Extend verification time of signup process",
        alias = "sp-extend-verify"
    )]
    ExtendVerificationTimeOfSignupProcess {
        request: String,
        token: Option<String>,
    },
    #[clap(
        about = "Extend completion time of signup process",
        alias = "sp-extend-complete"
    )]
    ExtendCompletionTimeOfSignupProcess {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Delete signup process", alias = "sp-delete")]
    DeleteSignupProcess {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Verify Email of signup process", alias = "sp-verify")]
    VerifyEmailOfSignupProcess {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Complete signup process", alias = "sp-complete")]
    CompleteSignupProcess {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Get state chain for signup process", alias = "sp-chain")]
    GetStateChain {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Login user")]
    Login {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "List all users")]
    ListUsers {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Read user")]
    ReadUser {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Update user")]
    UpdateUser {
        request: String,
        token: Option<String>,
    },
    #[clap(about = "Delete user")]
    DeleteUser {
        request: String,
        token: Option<String>,
    },
}

pub fn run<D>(db: Arc<D>, cmd: Command)
where
    D: Transactional,
{
    let app_controller = Controller::<D, boundary::Boundary>::new(db);

    match cmd {
        Command::InitializeSignupProcess { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<Initialize<D>>(request, token);
            println!("{res}");
        }
        Command::SendVerificationEmail { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<SendVerificationEmail<D>>(request, token);
            println!("{res}");
        }
        Command::ExtendVerificationTimeOfSignupProcess { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<ExtendVerificationTime<D>>(request, token);
            println!("{res}");
        }
        Command::ExtendCompletionTimeOfSignupProcess { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<ExtendCompletionTime<D>>(request, token);
            println!("{res}");
        }
        Command::DeleteSignupProcess { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<Delete<D>>(request, token);
            println!("{res}");
        }
        Command::VerifyEmailOfSignupProcess { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<VerifyEmail<D>>(request, token);
            println!("{res}");
        }
        Command::CompleteSignupProcess { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<Complete<D>>(request, token);
            println!("{res}");
        }
        Command::GetStateChain { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<GetStateChain<D>>(request, token);
            println!("{res}");
        }
        Command::Login { request, token: _ } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<Login<D>>(request, None);
            println!("{res}");
        }
        Command::ListUsers { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<GetAll<D>>(request, token);
            println!("{res}");
        }
        Command::DeleteUser { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<UserDelete<D>>(request, token);
            println!("{res}");
        }
        Command::ReadUser { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<GetOne<D>>(request, token);
            println!("{res}");
        }
        Command::UpdateUser { request, token } => {
            let request = serde_json::from_str(&request).expect("Failed to parse request");
            let res = app_controller.handle_usecase::<Update<D>>(request, token);
            println!("{res}");
        }
    }
}
