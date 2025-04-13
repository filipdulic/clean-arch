use ca_application::identifier::NewIdError;
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

mod models;
mod repositories;

#[derive(Debug, Clone)]
pub struct SqlxSqlite {
    pool: Pool<Sqlite>,
}

impl SqlxSqlite {
    pub async fn try_new(folder: &str) -> Result<Self, sqlx::Error> {
        let db_url = format!("sqlite://{}/sqlite.db", folder);
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            println!("Creating database {}", &db_url);
            match Sqlite::create_database(&db_url).await {
                Ok(_) => println!("Create db success"),
                Err(error) => panic!("error: {}", error),
            }
        } else {
            println!("Database already exists");
        }
        let pool = SqlitePool::connect(&db_url).await.unwrap();

        // let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        // println!("CARGO_MANIFEST_DIR: {}", crate_dir);
        let migrations =
            std::path::Path::new("./crates/infrastructure/persistance/sqlx_sqlite/migrations");
        //.join("./crates/infrastructure/persistance/sqlx_sqlite/migrations");
        let migration_results = sqlx::migrate::Migrator::new(migrations)
            .await
            .unwrap()
            .run(&pool)
            .await;
        match migration_results {
            Ok(_) => println!("Migration success"),
            Err(error) => {
                panic!("error: {}", error);
            }
        }

        Ok(Self { pool })
    }
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
    pub fn new_id_inner(&self) -> Result<uuid::Uuid, NewIdError> {
        Ok(uuid::Uuid::new_v4())
    }
}
