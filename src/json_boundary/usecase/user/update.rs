use crate::json_boundary::domain::UserId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: UserId,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound(UserId),
    UserNameMinLength { min: usize, actual: usize },
    UserNameMaxLength { max: usize, actual: usize },
    EmailMinLength { min: usize, actual: usize },
    EmailMaxLength { max: usize, actual: usize },
}
