use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub username: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    UserNameMinLength { min: usize, actual: usize },
    UserNameMaxLength { max: usize, actual: usize },
}
