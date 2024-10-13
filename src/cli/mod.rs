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

use crate::adapter::{api::Api, db::Db, presenter::cli::Presenter};

#[derive(Subcommand)]
pub enum Command {
    #[clap(about = "Initialize signup process", alias = "sp-init")]
    InitializeSignupProcess { username: String },
    #[clap(about = "Verify Email of signup process", alias = "sp-verify")]
    VerifyEmailOfSignupProcess { id: String },
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

pub fn run<D>(db: Arc<D>, cmd: Command)
where
    D: Db,
{
    let app_api = Api::new(db, Presenter);

    match cmd {
        Command::InitializeSignupProcess { username } => {
            let res = app_api.initialize_signup_process(username);
            println!("{res}");
        }
        Command::VerifyEmailOfSignupProcess { id } => {
            let res = app_api.verify_email_to_signup_process(&id);
            println!("{res}");
        }
        Command::CompleteSignupProcess {
            id,
            username,
            password,
        } => {
            let res = app_api.complete_signup_process(&id, username, password);
            println!("{res}");
        }
        Command::ListUsers => {
            let res = app_api.read_all_users();
            println!("{res}");
        }
        Command::DeleteUser { id } => {
            let res = app_api.delete_user(&id);
            println!("{res}");
        }
        Command::ReadUser { id } => {
            let res = app_api.get_one_user(&id);
            println!("{res}");
        }
        Command::UpdateUser {
            id,
            email,
            username,
            password,
        } => {
            let res = app_api.update_user(&id, email, username, password);
            println!("{res}");
        }
    }
}
