use std::sync::Arc;

pub mod signup_process;
pub mod token;
pub mod user;

#[derive(Debug, Clone)]
pub struct SqlxSqliteRepository {
    pool: Arc<sqlx::SqlitePool>,
}

impl SqlxSqliteRepository {
    pub fn new(pool: Arc<sqlx::SqlitePool>) -> Self {
        Self { pool }
    }
}
