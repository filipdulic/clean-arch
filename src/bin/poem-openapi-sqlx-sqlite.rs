use std::sync::Arc;

use ca_application::gateway::{
    service::{
        auth::{AuthExtractor, AuthPacker},
        email::EmailVerificationService,
    },
    AuthExtractorProvider, AuthPackerProvider, DatabaseProvider, EmailVerificationServiceProvider,
};
use ca_infrastructure_auth_jwt::JwtAuth;
use ca_infrastructure_interface_poem_openapi::Api;
use ca_infrastructure_persistance_sqlx_sqlite::SqlxSqlite;
use ca_infrastructure_service_email_file::{data_storage_directory, FileEmailService};
use poem::{listener::TcpListener, Route, Server};
use poem_openapi::OpenApiService;

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
    fn auth_packer(&self) -> impl AuthPacker {
        &self.jwt_auth
    }
}

#[tokio::main]
async fn main() {
    let data_folder_path = data_storage_directory(None);
    let data_folder_str = data_folder_path.to_str().unwrap();
    let email_verification_service = FileEmailService::try_new(data_folder_path.clone()).unwrap();
    let jwt_auth = JwtAuth::new("secret".to_string());
    let sqlx_sqlite = SqlxSqlite::try_new(data_folder_str).await.unwrap();
    let dep_provider = Arc::new(DependancyProvider::new(
        sqlx_sqlite,
        email_verification_service,
        jwt_auth,
    ));
    let api_service = OpenApiService::new(Api::new(dep_provider), "Hello World", "1.0")
        .server("http://localhost:3000");
    let ui = api_service.swagger_ui();
    let app = Route::new().nest("/", api_service).nest("/docs", ui);

    Server::new(TcpListener::bind("127.0.0.1:3000"))
        .run(app)
        .await
        .unwrap();
}
