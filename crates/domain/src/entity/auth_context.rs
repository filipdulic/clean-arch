use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::entity::user::Id as UserId;
use crate::value_object::Role;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthContext {
    pub user_id: UserId,
    pub role: Role,
}

impl AuthContext {
    pub fn new(user_id: UserId, role: Role) -> Self {
        Self { user_id, role }
    }

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn role(&self) -> &Role {
        &self.role
    }

    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Error)]
pub enum AuthError {
    #[error("Unauthorized")]
    Unauthorized,
}
