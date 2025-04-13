use ca_application::gateway::repository::user::Record;
use ca_domain::entity::user::{Email, Password, User as DomainUser, UserName};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    id: String,
    #[sqlx(rename = "name")]
    username: String,
    email: String,
    #[sqlx(rename = "password")]
    password_hash: String,
    role: String,
}

impl From<Record> for User {
    fn from(record: Record) -> Self {
        Self {
            id: record.user.id().to_string(),
            username: record.user.username().to_string(),
            email: record.user.email().to_string(),
            password_hash: record.user.password().to_string(),
            role: record.user.role().to_string(),
        }
    }
}

impl From<User> for Record {
    fn from(user: User) -> Self {
        let id = Uuid::parse_str(&user.id).unwrap();
        let role = user.role.parse().unwrap();
        let email = Email::new(user.email);
        let username = UserName::new(user.username);
        let password_hash = Password::new(user.password_hash);

        Record {
            user: DomainUser::new(id.into(), role, email, username, password_hash),
        }
    }
}
