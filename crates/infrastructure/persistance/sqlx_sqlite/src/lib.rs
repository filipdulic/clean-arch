use futures::lock::Mutex;
use repositories::SqlxSqliteRepository;
use std::sync::Arc;

use ca_application::gateway::database::{self, identifier::NewId, Database};
use ca_domain::{entity::signup_process::SignupProcessValue, value_object::Id};
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};

mod models;
mod repositories;

#[derive(Debug, Clone)]
pub struct SqlxSqlite {
    pool: Arc<Pool<Sqlite>>,
    repositories: Arc<SqlxSqliteRepository>,
}

pub type SqlxSqliteTransaction = sqlx::Transaction<'static, Sqlite>;

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
        let arc_pool = Arc::new(pool);

        Ok(Self {
            pool: arc_pool.clone(),
            repositories: Arc::new(SqlxSqliteRepository::new(arc_pool)),
        })
    }
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
}
#[async_trait::async_trait]
impl Database for SqlxSqlite {
    type Error = ();
    type Transaction = SqlxSqliteTransaction;

    async fn begin_transaction(&self) -> Arc<Mutex<Self::Transaction>> {
        Arc::new(Mutex::new(
            self.pool()
                .begin()
                .await
                .expect("Failed to begin transaction"),
        ))
    }

    async fn commit_transaction(
        &self,
        transaction: Arc<Mutex<Self::Transaction>>,
    ) -> Result<(), Self::Error> {
        Arc::try_unwrap(transaction)
            .unwrap()
            .into_inner()
            .commit()
            .await
            .map_err(|err| {
                println!("Transaction commit error: {:?}", err);
            })
    }

    async fn rollback_transaction(
        &self,
        transaction: Arc<Mutex<Self::Transaction>>,
    ) -> Result<(), Self::Error> {
        Arc::try_unwrap(transaction)
            .unwrap()
            .into_inner()
            .rollback()
            .await
            .map_err(|err| {
                println!("Transaction rollback error: {:?}", err);
            })
    }

    fn signup_process_repo(
        &self,
    ) -> Arc<dyn database::signup_process::Repo<Transaction = Self::Transaction> + Send + Sync>
    {
        self.repositories.clone()
    }

    fn signuo_id_gen(&self) -> Arc<dyn NewId<Id<SignupProcessValue>> + Send + Sync> {
        self.repositories.clone()
    }

    fn user_repo(
        &self,
    ) -> Arc<dyn database::user::Repo<Transaction = Self::Transaction> + Send + Sync> {
        self.repositories.clone()
    }

    fn token_repo(
        &self,
    ) -> Arc<dyn database::token::Repo<Transaction = Self::Transaction> + Send + Sync> {
        self.repositories.clone()
    }
}
