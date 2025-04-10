use ca_adapter::dependency_provider::Transactional;
use ca_application::gateway::service::email::EmailVerificationService;
use ca_application::gateway::{
    AuthExtractorProvider, AuthPackerProvider, EmailVerificationServiceProvider,
    SignupProcessIdGenProvider, SignupProcessRepoProvider, TokenRepoProvider, UserIdGenProvider,
    UserRepoProvider,
};
use ca_application::identifier::NewId;
use ca_application::usecase::Comitable;
use ca_domain::entity::{signup_process::Id as SignupProcessId, user::Id as UserId};
use ca_infrastructure_auth_jwt::JwtAuth;
use ca_infrastructure_interface_cli as cli;
use ca_infrastructure_persistance_json_file::utils::{data_storage, data_storage_directory};
use ca_infrastructure_persistance_json_file::JsonFile;
use ca_infrastructure_service_email_file::FileEmailService;
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
    jwt_auth: JwtAuth,
}

impl DependancyProvider {
    fn new(db: JsonFile, email_verification_servuce: FileEmailService, jwt_auth: JwtAuth) -> Self {
        Self {
            db,
            email_verification_servuce,
            jwt_auth,
        }
    }
}

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
            jwt_auth: self.jwt_auth.clone(),
        }
    }
}

impl EmailVerificationServiceProvider for DependancyProvider {
    fn email_verification_service(&self) -> &dyn EmailVerificationService {
        &self.email_verification_servuce
    }
}

impl AuthExtractorProvider for DependancyProvider {
    fn auth_extractor(&self) -> &dyn ca_application::gateway::service::auth::AuthExtractor {
        &self.jwt_auth
    }
}

impl AuthPackerProvider for DependancyProvider {
    fn auth_packer(&self) -> &dyn ca_application::gateway::service::auth::AuthPacker {
        &self.jwt_auth
    }
}

impl Transactional for DependancyProvider {
    // TODO: implement a proper transaction
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> Result<R, E>
    where
        Result<R, E>: Into<Comitable<R, E>>,
        F: FnOnce(&'d Self) -> Result<R, E>,
    {
        let res = f(self);
        match res.into() {
            Comitable::Commit(inner) => {
                println!("Commiting");
                inner
            }
            Comitable::Rollback(inner) => {
                println!("Rolling Back");
                inner
            }
        }
    }
}

pub fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let email_folder = data_storage_directory(None);
    let email_verification_servuce = FileEmailService::try_new(email_folder)?;
    let jwt_auth = JwtAuth::new("secret".to_string());
    let dep_provider = Arc::new(DependancyProvider::new(
        data_storage(args.data_dir),
        email_verification_servuce,
        jwt_auth,
    ));
    cli::run(dep_provider, args.command);
    Ok(())
}
