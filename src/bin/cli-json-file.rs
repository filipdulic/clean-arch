use ca_adapter::dependency_provider::Transactional;
use ca_application::gateway::service::email::EmailVerificationService;
use ca_application::gateway::{
    EmailVerificationServiceProvider, SignupProcessIdGenProvider, SignupProcessRepoProvider,
    TokenRepoProvider, UserIdGenProvider, UserRepoProvider,
};
use ca_application::identifier::NewId;
use ca_infrastructure::service::email::file::FileEmailService;
use ca_infrastructure::utils::storage::{data_storage, data_storage_directory};
use ca_infrastructure::{interface::cli, persistance::json_file::JsonFile};
use clap::Parser;
use std::{path::PathBuf, sync::Arc};

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: cli::Command,
    #[clap(help = "Directory to store data ", long)]
    data_dir: Option<PathBuf>,
}

struct DependancyProvider {
    db: JsonFile,
    email_verification_servuce: FileEmailService,
}

impl DependancyProvider {
    fn new(db: JsonFile, email_verification_servuce: FileEmailService) -> Self {
        Self {
            db,
            email_verification_servuce,
        }
    }
}

use ca_domain::entity::{signup_process::Id as SignupProcessId, user::Id as UserId};

impl SignupProcessIdGenProvider for DependancyProvider {
    fn signup_process_id_gen(&self) -> &dyn NewId<SignupProcessId> {
        &self.db
    }
}

impl SignupProcessRepoProvider for DependancyProvider {
    fn signup_process_repo(
        &self,
    ) -> &dyn ca_application::gateway::repository::signup_process::Repo {
        &self.db
    }
}

impl UserIdGenProvider for DependancyProvider {
    fn user_id_gen(&self) -> &dyn NewId<UserId> {
        &self.db
    }
}

impl UserRepoProvider for DependancyProvider {
    fn user_repo(&self) -> &dyn ca_application::gateway::repository::user::Repo {
        &self.db
    }
}

impl TokenRepoProvider for DependancyProvider {
    fn token_repo(&self) -> &dyn ca_application::gateway::repository::token::Repo {
        &self.db
    }
}

impl Clone for DependancyProvider {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            email_verification_servuce: self.email_verification_servuce.clone(),
        }
    }
}

impl EmailVerificationServiceProvider for DependancyProvider {
    fn email_verification_service(&self) -> &dyn EmailVerificationService {
        &self.email_verification_servuce
    }
}

impl Transactional for DependancyProvider {
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> Result<R, E>
    where
        F: FnOnce(&'d Self) -> Result<R, E>,
    {
        f(self)
    }
}

pub fn main() {
    let args = Args::parse();
    let email_folder = data_storage_directory(None);
    let email_verification_servuce = FileEmailService::try_new(email_folder).unwrap();
    let dep_provider = Arc::new(DependancyProvider::new(
        data_storage(args.data_dir),
        email_verification_servuce,
    ));
    cli::run(dep_provider, args.command);
}
