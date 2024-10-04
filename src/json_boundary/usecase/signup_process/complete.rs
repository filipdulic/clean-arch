use crate::json_boundary::domain::SignupProcessId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub id: SignupProcessId,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Error {
    Id,
    NotFound,
}
