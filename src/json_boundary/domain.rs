use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignupState {
    Initialized,
    EmailAdded,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignupProcess {
    pub id: SignupProcessId,
    pub chain: Vec<SignupState>,
    pub state: SignupState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: UserId,
    pub username: String,
    pub email: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SignupProcessId(pub Uuid);

impl From<Uuid> for SignupProcessId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl fmt::Display for SignupProcessId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub Uuid);

impl From<Uuid> for UserId {
    fn from(id: Uuid) -> Self {
        Self(id)
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
