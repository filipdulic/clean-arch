use ca_application::gateway::database::{self, Database, identifier::NewId};
use ca_domain::{entity::signup_process::SignupProcessValue, value_object::Id};
use std::path::Path;

use sqlx::{
    Pool, Sqlite,
    migrate::{MigrateError, Migrator},
    sqlite::{SqliteConnectOptions, SqlitePoolOptions},
};
use thiserror::Error;

mod models;
mod repositories;

#[derive(Debug, Clone)]
pub struct SqlxSqlite {
    pool: Pool<Sqlite>,
}

pub type SqlxSqliteTransaction = sqlx::Transaction<'static, Sqlite>;

static MIGRATOR: Migrator = sqlx::migrate!();

#[derive(Debug, Error)]
pub enum InitError {
    #[error("Unable to create SQLite data directory")]
    CreateDirectory(#[source] std::io::Error),
    #[error("Unable to connect to SQLite")]
    Connect(#[source] sqlx::Error),
    #[error("Unable to apply SQLite migrations")]
    Migrate(#[source] MigrateError),
}

impl SqlxSqlite {
    pub async fn try_new(folder: impl AsRef<Path>) -> Result<Self, InitError> {
        tokio::fs::create_dir_all(folder.as_ref())
            .await
            .map_err(InitError::CreateDirectory)?;
        let options = SqliteConnectOptions::new()
            .filename(folder.as_ref().join("sqlite.db"))
            .create_if_missing(true);
        let pool = SqlitePoolOptions::new()
            .connect_with(options)
            .await
            .map_err(InitError::Connect)?;
        MIGRATOR.run(&pool).await.map_err(InitError::Migrate)?;

        Ok(Self { pool })
    }
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
    pub fn new_id_inner(&self) -> uuid::Uuid {
        uuid::Uuid::new_v4()
    }
}
#[async_trait::async_trait]
impl Database for &SqlxSqlite {
    type Error = ();
    type Transaction = SqlxSqliteTransaction;

    async fn begin_transaction(&self) -> Self::Transaction {
        self.pool()
            .begin()
            .await
            .expect("Failed to begin transaction")
    }

    async fn commit_transaction(&self, transaction: Self::Transaction) -> Result<(), Self::Error> {
        transaction.commit().await.map_err(|err| {
            println!("Transaction commit error: {:?}", err);
        })
    }

    async fn rollback_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> Result<(), Self::Error> {
        transaction.rollback().await.map_err(|err| {
            println!("Transaction rollback error: {:?}", err);
        })
    }

    fn signup_process_repo(
        &self,
    ) -> impl database::signup_process::Repo<Transaction = Self::Transaction> {
        *self
    }

    fn signup_id_gen(&self) -> impl NewId<Id<SignupProcessValue>> {
        *self
    }

    fn user_repo(&self) -> impl database::user::Repo<Transaction = Self::Transaction> {
        *self
    }

    fn token_repo(&self) -> impl database::token::Repo<Transaction = Self::Transaction> {
        *self
    }
}

#[cfg(test)]
mod tests {
    use ca_application::gateway::database::{
        signup_process::{GetError as SignupGetError, Repo as SignupRepo},
        token::{ExtendError, Repo as TokenRepo},
        user::{DeleteError, GetError as UserGetError, Repo as UserRepo},
    };
    use ca_domain::{
        entity::{
            signup_process::{Id as SignupId, SignupProcess, SignupStateEnum},
            user::{Email, Id as UserId, Password, User, UserName},
        },
        value_object::Role,
    };

    use super::SqlxSqlite;

    #[tokio::test]
    async fn updates_and_deletes_users() {
        let directory = tempfile::tempdir().unwrap();
        let database = SqlxSqlite::try_new(directory.path()).await.unwrap();
        let id = UserId::from(uuid::Uuid::new_v4());
        let mut user = User::new(
            id,
            Role::User,
            Email::new("first@example.com"),
            UserName::new("first-user"),
            Password::new("first-password"),
        );
        (&database).save(None, user.clone().into()).await.unwrap();

        user.update(
            Email::new("second@example.com"),
            UserName::new("second-user"),
            Password::new("second-password"),
        );
        (&database).save(None, user.clone().into()).await.unwrap();

        let stored = (&database).get(None, id).await.unwrap();
        assert_eq!(stored.user, user);
        UserRepo::delete(&&database, None, id).await.unwrap();
        assert!(matches!(
            UserRepo::delete(&&database, None, id).await,
            Err(DeleteError::NotFound)
        ));

        sqlx::query("INSERT INTO users (id, name, email, password, role) VALUES (?, ?, ?, ?, ?)")
            .bind(id.to_string())
            .bind("invalid-user")
            .bind("invalid@example.com")
            .bind("invalid-password")
            .bind("invalid-role")
            .execute(database.pool())
            .await
            .unwrap();
        assert!(matches!(
            (&database).get(None, id).await,
            Err(UserGetError::InvalidData)
        ));
    }

    #[tokio::test]
    async fn reads_signup_states_in_order() {
        let directory = tempfile::tempdir().unwrap();
        let database = SqlxSqlite::try_new(directory.path()).await.unwrap();
        let id = SignupId::from(uuid::Uuid::new_v4());
        assert!(matches!(
            (&database).get_latest_state(None, id).await,
            Err(SignupGetError::NotFound)
        ));
        assert!(matches!(
            (&database).get_state_chain(None, id).await,
            Err(SignupGetError::NotFound)
        ));

        let initialized = SignupProcess::new(id, Email::new("user@example.com"));
        (&database)
            .save_latest_state(None, initialized.clone().into())
            .await
            .unwrap();
        (&database)
            .save_latest_state(None, initialized.send_verification_email().into())
            .await
            .unwrap();

        let states = (&database).get_state_chain(None, id).await.unwrap();
        assert!(matches!(
            states[0].state,
            SignupStateEnum::Initialized { .. }
        ));
        assert!(matches!(
            states[1].state,
            SignupStateEnum::VerificationEmailSent { .. }
        ));
    }

    #[tokio::test]
    async fn reports_missing_token_on_extend() {
        let directory = tempfile::tempdir().unwrap();
        let database = SqlxSqlite::try_new(directory.path()).await.unwrap();
        assert!(matches!(
            (&database).extend(None, "missing@example.com").await,
            Err(ExtendError::NotFound)
        ));
    }
}
