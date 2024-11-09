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
        complete::{Complete, Request as CompleteRequest},
        completion_timed_out::{CompletionTimedOut, Request as CompletionTimedOutRequest},
        delete::{Delete, Request as DeleteRequest},
        extend_completion_time::{ExtendCompletionTime, Request as ExtendCompletionTimeRequest},
        extend_verification_time::{
            ExtendVerificationTime, Request as ExtendVerificationTimeRequest,
        },
        get_state_chain::{GetStateChain, Request as GetStateChainRequest},
        initialize::{Initialize, Request as InitializeRequest},
        send_verification_email::{Request as SendVerificationEmailRequest, SendVerificationEmail},
        verify_email::{Request as VerifyEmailRequest, VerifyEmail},
    },
    user::{
        delete::{Delete as UserDelete, Request as UserDeleteRequest},
        get_all::{GetAll, Request as GetAllRequest},
        get_one::{GetOne, Request as GetOneRequest},
        update::{Request as UpdateRequest, Update},
    },
};
use uuid::Uuid;

//use crate::boundary::string::
#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Initialize signup process", alias = "sp-init")]
    InitializeSignupProcess { email: String },
    #[clap(
        about = "Send verification email for signup process",
        alias = "sp-send-verify"
    )]
    SendVerificationEmail { id: Uuid },
    #[clap(
        about = "Signup process completion timed out",
        alias = "sp-complete-timeout"
    )]
    SignupProcessCompletionTimedOut { id: Uuid },
    #[clap(
        about = "Extend verification time of signup process",
        alias = "sp-extend-verify"
    )]
    ExtendVerificationTimeOfSignupProcess { id: Uuid },
    #[clap(
        about = "Extend completion time of signup process",
        alias = "sp-extend-complete"
    )]
    ExtendCompletionTimeOfSignupProcess { id: Uuid },
    #[clap(about = "Delete signup process", alias = "sp-delete")]
    DeleteSignupProcess { id: Uuid },
    #[clap(about = "Verify Email of signup process", alias = "sp-verify")]
    VerifyEmailOfSignupProcess { id: Uuid, token: String },
    #[clap(about = "Complete signup process", alias = "sp-complete")]
    CompleteSignupProcess {
        id: Uuid,
        username: String,
        password: String,
    },
    #[clap(about = "Get state chain for signup process", alias = "sp-chain")]
    GetStateChain { id: Uuid },
    #[clap(about = "List all users")]
    ListUsers,
    #[clap(about = "Read user")]
    ReadUser { id: Uuid },
    #[clap(about = "Update user")]
    UpdateUser {
        id: Uuid,
        email: String,
        username: String,
        password: String,
    },
    #[clap(about = "Delete user")]
    DeleteUser { id: Uuid },
}

pub fn run<D>(db: Arc<D>, cmd: Command)
where
    D: Transactional,
{
    let app_controller = Controller::<D>::new(db);

    match cmd {
        Command::InitializeSignupProcess { email } => {
            let res = app_controller.handle_usecase::<Initialize<D>>(InitializeRequest { email });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::SendVerificationEmail { id } => {
            let res = app_controller.handle_usecase::<SendVerificationEmail<D>>(
                SendVerificationEmailRequest { id: id.into() },
            );
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::SignupProcessCompletionTimedOut { id } => {
            let res =
                app_controller.handle_usecase::<CompletionTimedOut<D>>(CompletionTimedOutRequest {
                    id: id.into(),
                });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::ExtendVerificationTimeOfSignupProcess { id } => {
            let res = app_controller.handle_usecase::<ExtendVerificationTime<D>>(
                ExtendVerificationTimeRequest { id: id.into() },
            );
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::ExtendCompletionTimeOfSignupProcess { id } => {
            let res = app_controller.handle_usecase::<ExtendCompletionTime<D>>(
                ExtendCompletionTimeRequest { id: id.into() },
            );
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::DeleteSignupProcess { id } => {
            let res = app_controller.handle_usecase::<Delete<D>>(DeleteRequest { id: id.into() });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::VerifyEmailOfSignupProcess { id, token } => {
            let res = app_controller.handle_usecase::<VerifyEmail<D>>(VerifyEmailRequest {
                id: id.into(),
                token,
            });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::CompleteSignupProcess {
            id,
            username,
            password,
        } => {
            let res = app_controller.handle_usecase::<Complete<D>>(CompleteRequest {
                id: id.into(),
                username,
                password,
            });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::GetStateChain { id } => {
            let res = app_controller
                .handle_usecase::<GetStateChain<D>>(GetStateChainRequest { id: id.into() });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::ListUsers => {
            let res = app_controller.handle_usecase::<GetAll<D>>(GetAllRequest {});
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::DeleteUser { id } => {
            let res =
                app_controller.handle_usecase::<UserDelete<D>>(UserDeleteRequest { id: id.into() });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::ReadUser { id } => {
            let res = app_controller.handle_usecase::<GetOne<D>>(GetOneRequest { id: id.into() });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
        Command::UpdateUser {
            id,
            email,
            username,
            password,
        } => {
            let res = app_controller.handle_usecase::<Update<D>>(UpdateRequest {
                id: id.into(),
                email,
                username,
                password,
            });
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        }
    };
}
