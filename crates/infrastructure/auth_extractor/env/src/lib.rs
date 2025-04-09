use std::str::FromStr;
use uuid::Uuid;

use ca_adapter::auth_extractor::AuthExtractor;
use ca_domain::{entity::auth_context::AuthContext, value_object::Role};

pub struct EnvAuthExtractor;

impl AuthExtractor for EnvAuthExtractor {
    type AuthInput = ();

    fn extract_auth(&self, _: Self::AuthInput) -> Option<AuthContext> {
        let user_id = std::env::var("AUTH_USER_ID")
            .ok()
            .and_then(|s| Uuid::parse_str(&s).ok())
            .map(ca_domain::entity::user::Id::from);
        let role = std::env::var("AUTH_ROLE")
            .ok()
            .and_then(|s| Role::from_str(&s).ok());
        match (user_id, role) {
            (Some(user_id), Some(role)) => Some(AuthContext { user_id, role }),
            _ => None,
        }
    }
}
