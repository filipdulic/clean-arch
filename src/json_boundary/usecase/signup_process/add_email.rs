use crate::json_boundary::domain::SignupProcessId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: SignupProcessId,
    pub email: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound,
    EmailMinLength { min: usize, actual: usize },
    EmailMaxLength { max: usize, actual: usize },
}
