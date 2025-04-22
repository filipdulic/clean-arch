use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AuthStrategy {
    // Allow access only to admin
    AdminOnly,
    // Allow access only to owner and admin
    AdminAndOwnerOnly,
    // Allow access to anyone
    Public,
}
