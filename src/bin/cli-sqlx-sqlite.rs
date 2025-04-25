use ca_application::gateway::database::Database;
use ca_application::gateway::service::auth::{AuthExtractor, AuthPacker};
use ca_application::gateway::service::email::EmailVerificationService;
use ca_application::gateway::{
    AuthExtractorProvider, AuthPackerProvider, DatabaseProvider, EmailVerificationServiceProvider,
};

use ca_infrastructure_auth_jwt::JwtAuth;
use ca_infrastructure_interface_cli as cli;
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
    db: Arc<SqlxSqlite>,
    email_verification_servuce: Arc<FileEmailService>,
    jwt_auth: Arc<JwtAuth>,
}

impl DependancyProvider {
    fn new(
        db: Arc<SqlxSqlite>,
        email_verification_servuce: Arc<FileEmailService>,
        jwt_auth: Arc<JwtAuth>,
    ) -> Self {
        Self {
            db,
            email_verification_servuce,
            jwt_auth,
        }
    }
}

impl DatabaseProvider for DependancyProvider {
    type Transaction = ca_infrastructure_persistance_sqlx_sqlite::SqlxSqliteTransaction;
    type Error = ();
    fn database(
        &self,
    ) -> Arc<dyn Database<Transaction = Self::Transaction, Error = Self::Error> + Send + Sync> {
        self.db.clone()
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
    fn email_verification_service(&self) -> Arc<dyn EmailVerificationService + Send + Sync> {
        self.email_verification_servuce.clone()
    }
}

impl AuthExtractorProvider for DependancyProvider {
    fn auth_extractor(&self) -> Arc<dyn AuthExtractor + Send + Sync> {
        self.jwt_auth.clone()
    }
}

impl AuthPackerProvider for DependancyProvider {
    fn auth_packer(&self) -> Arc<dyn AuthPacker + Send + Sync> {
        self.jwt_auth.clone()
    }
}

#[tokio::main]
pub async fn main() -> Result<(), std::io::Error> {
    let args = Args::parse();
    let data_folder_path = data_storage_directory(None);
    let data_folder_str = data_folder_path.to_str().unwrap();
    let email_verification_service = FileEmailService::try_new(data_folder_path.clone())?;
    let jwt_auth = JwtAuth::new("secret".to_string());
    let sqlx_sqlite = SqlxSqlite::try_new(data_folder_str).await.unwrap();
    let dep_provider = Arc::new(DependancyProvider::new(
        Arc::new(sqlx_sqlite),
        Arc::new(email_verification_service),
        Arc::new(jwt_auth),
    ));
    cli::run(dep_provider, args.command).await;
    Ok(())
}
