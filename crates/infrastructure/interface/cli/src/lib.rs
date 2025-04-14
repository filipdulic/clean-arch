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
//!   and match them to the defined commands.
//! * Command Execution: Map the parsed commands to the appropriate functions or
//!   methods in the application.
use std::sync::Arc;

use clap::Subcommand;

use ca_adapter::controller::Controller;
use ca_application::{
    gateway::{
        AuthExtractorProvider, AuthPackerProvider, DatabaseProvider,
        EmailVerificationServiceProvider,
    },
    usecase::{
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
    },
};

use ca_infrastructure_boundary_string as string;

//use crate::boundary::string::
#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Initialize signup process", alias = "sp-init")]
    InitializeSignupProcess {
        email: String,
        token: Option<String>,
    },
    #[clap(
        about = "Send verification email for signup process",
        alias = "sp-send-verify"
    )]
    SendVerificationEmail { id: String, token: Option<String> },
    #[clap(
        about = "Extend verification time of signup process",
        alias = "sp-extend-verify"
    )]
    ExtendVerificationTimeOfSignupProcess { id: String, token: Option<String> },
    #[clap(
        about = "Extend completion time of signup process",
        alias = "sp-extend-complete"
    )]
    ExtendCompletionTimeOfSignupProcess { id: String, token: Option<String> },
    #[clap(about = "Delete signup process", alias = "sp-delete")]
    DeleteSignupProcess { id: String, token: Option<String> },
    #[clap(about = "Verify Email of signup process", alias = "sp-verify")]
    VerifyEmailOfSignupProcess {
        id: String,
        signup_token: String,
        token: Option<String>,
    },
    #[clap(about = "Complete signup process", alias = "sp-complete")]
    CompleteSignupProcess {
        id: String,
        username: String,
        password: String,
        token: Option<String>,
    },
    #[clap(about = "Get state chain for signup process", alias = "sp-chain")]
    GetStateChain { id: String, token: Option<String> },
    #[clap(about = "Login user")]
    Login { username: String, password: String },
    #[clap(about = "List all users")]
    ListUsers { token: Option<String> },
    #[clap(about = "Read user")]
    ReadUser { id: String, token: Option<String> },
    #[clap(about = "Update user")]
    UpdateUser {
        id: String,
        email: String,
        username: String,
        password: String,
        token: Option<String>,
    },
    #[clap(about = "Delete user")]
    DeleteUser { id: String, token: Option<String> },
}

pub async fn run<D>(db: Arc<D>, cmd: Command)
where
    D: DatabaseProvider
        + EmailVerificationServiceProvider
        + AuthPackerProvider
        + AuthExtractorProvider,
{
    let app_controller = Controller::<D, string::Boundary>::new(db);

    match cmd {
        Command::InitializeSignupProcess { email, token } => {
            let res = app_controller
                .handle_usecase::<Initialize<D>>(email, token)
                .await;
            println!("{res}");
        }
        Command::SendVerificationEmail { id, token } => {
            let res = app_controller
                .handle_usecase::<SendVerificationEmail<D>>(id, token)
                .await;
            println!("{res}");
        }
        Command::ExtendVerificationTimeOfSignupProcess { id, token } => {
            let res = app_controller
                .handle_usecase::<ExtendVerificationTime<D>>(id, token)
                .await;
            println!("{res}");
        }
        Command::ExtendCompletionTimeOfSignupProcess { id, token } => {
            let res = app_controller
                .handle_usecase::<ExtendCompletionTime<D>>(id, token)
                .await;
            println!("{res}");
        }
        Command::DeleteSignupProcess { id, token } => {
            let res = app_controller.handle_usecase::<Delete<D>>(id, token).await;
            println!("{res}");
        }
        Command::VerifyEmailOfSignupProcess {
            id,
            signup_token,
            token,
        } => {
            let res = app_controller
                .handle_usecase::<VerifyEmail<D>>((id, signup_token), token)
                .await;
            println!("{res}");
        }
        Command::CompleteSignupProcess {
            id,
            username,
            password,
            token,
        } => {
            let res = app_controller
                .handle_usecase::<Complete<D>>((id, username, password), token)
                .await;
            println!("{res}");
        }
        Command::GetStateChain { id, token } => {
            let res = app_controller
                .handle_usecase::<GetStateChain<D>>(id, token)
                .await;
            println!("{res}");
        }
        Command::Login { username, password } => {
            let res = app_controller
                .handle_usecase::<Login<D>>((username, password), None)
                .await;
            println!("{res}");
        }
        Command::ListUsers { token } => {
            let res = app_controller.handle_usecase::<GetAll<D>>((), token).await;
            println!("{res}");
        }
        Command::DeleteUser { id, token } => {
            let res = app_controller
                .handle_usecase::<UserDelete<D>>(id, token)
                .await;
            println!("{res}");
        }
        Command::ReadUser { id, token } => {
            let res = app_controller.handle_usecase::<GetOne<D>>(id, token).await;
            println!("{res}");
        }
        Command::UpdateUser {
            id,
            email,
            username,
            password,
            token,
        } => {
            let res = app_controller
                .handle_usecase::<Update<D>>((id, email, username, password), token)
                .await;
            println!("{res}");
        }
    }
}
