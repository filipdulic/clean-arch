use ca_application::gateway::service::auth::AuthExtractor;
use ca_application::gateway::service::email::EmailVerificationService;
use ca_application::gateway::{
    AuthExtractorProvider, AuthPackerProvider, DatabaseProvider, EmailVerificationServiceProvider,
};
use ca_infrastructure_auth_jwt::JwtAuth;
use ca_infrastructure_interface_cli_json as cli;
use ca_infrastructure_persistance_sqlx_sqlite::SqlxSqlite;
use ca_infrastructure_service_email_file::{data_storage_directory, FileEmailService};
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
    db: SqlxSqlite,
    email_verification_servuce: FileEmailService,
    jwt_auth: JwtAuth,
}

impl DependancyProvider {
    fn new(
        db: SqlxSqlite,
        email_verification_servuce: FileEmailService,
        jwt_auth: JwtAuth,
    ) -> Self {
        Self {
            db,
            email_verification_servuce,
            jwt_auth,
        }
    }
}

impl DatabaseProvider for DependancyProvider {
    fn database(&self) -> impl ca_application::gateway::database::Database {
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
    fn email_verification_service(&self) -> impl EmailVerificationService {
        &self.email_verification_servuce
    }
}

impl AuthExtractorProvider for DependancyProvider {
    fn auth_extractor(&self) -> impl AuthExtractor {
        &self.jwt_auth
    }
}

impl AuthPackerProvider for DependancyProvider {
    fn auth_packer(&self) -> impl ca_application::gateway::service::auth::AuthPacker {
        &self.jwt_auth
    }
}
#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let data_folder_path = data_storage_directory(None);
    let data_folder_str = data_folder_path.to_str().unwrap();
    let email_verification_servuce = FileEmailService::try_new(data_folder_path.clone())?;
    let jwt_auth = JwtAuth::new("secret".to_string());
    let sqlx_sqlite = SqlxSqlite::try_new(data_folder_str).await.unwrap();
    let dep_provider = Arc::new(DependancyProvider::new(
        sqlx_sqlite,
        email_verification_servuce,
        jwt_auth,
    ));
    cli::run(dep_provider, args.command).await;
    Ok(())
}
