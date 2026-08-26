use ca_application::gateway::service::auth::{AuthExtractor, AuthPacker};
use ca_application::gateway::service::email::EmailVerificationService;
use ca_application::gateway::{
    AuthExtractorProvider, AuthPackerProvider, DatabaseProvider, EmailVerificationServiceProvider,
};

use ca_infrastructure_auth_jwt::JwtAuth;
use ca_infrastructure_interface_cli as cli;
use ca_infrastructure_persistence_sqlx_sqlite::SqlxSqlite;
use ca_infrastructure_service_email_file::{FileEmailService, data_storage_directory};
use clap::Parser;
use std::{path::PathBuf, sync::Arc};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: cli::Command,
    #[arg(help = "Directory used to store data", long)]
    data_dir: Option<PathBuf>,
}

struct DependencyProvider {
    db: SqlxSqlite,
    email_verification_service: FileEmailService,
    jwt_auth: JwtAuth,
}

impl DependencyProvider {
    fn new(
        db: SqlxSqlite,
        email_verification_service: FileEmailService,
        jwt_auth: JwtAuth,
    ) -> Self {
        Self {
            db,
            email_verification_service,
            jwt_auth,
        }
    }
}

impl DatabaseProvider for DependencyProvider {
    fn database(&self) -> impl ca_application::gateway::database::Database {
        &self.db
    }
}

impl Clone for DependencyProvider {
    fn clone(&self) -> Self {
        Self {
            db: self.db.clone(),
            email_verification_service: self.email_verification_service.clone(),
            jwt_auth: self.jwt_auth.clone(),
        }
    }
}

impl EmailVerificationServiceProvider for DependencyProvider {
    fn email_verification_service(&self) -> impl EmailVerificationService {
        &self.email_verification_service
    }
}

impl AuthExtractorProvider for DependencyProvider {
    fn auth_extractor(&self) -> impl AuthExtractor {
        &self.jwt_auth
    }
}

impl AuthPackerProvider for DependencyProvider {
    fn auth_packer(&self) -> impl AuthPacker {
        &self.jwt_auth
    }
}

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let data_folder_path = data_storage_directory(args.data_dir);
    let email_verification_service = FileEmailService::try_new(data_folder_path.clone())?;
    let jwt_auth = JwtAuth::new("secret".to_string());
    let sqlx_sqlite = SqlxSqlite::try_new(data_folder_path).await?;
    let dep_provider = Arc::new(DependencyProvider::new(
        sqlx_sqlite,
        email_verification_service,
        jwt_auth,
    ));
    cli::run(dep_provider, args.command).await;
    Ok(())
}
