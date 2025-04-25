use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use uuid::Uuid;

use ca_application::gateway::service::auth::{AuthExtractor, AuthPacker};
use ca_domain::{entity::auth_context::AuthContext, value_object::Role};

#[derive(Clone)]
pub struct JwtAuth {
    secret: String,
}

impl JwtAuth {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    exp: usize,
    user_id: String,
    role: String,
}
impl Claims {
    fn new(auth_context: AuthContext) -> Self {
        Self {
            exp: (Utc::now() + chrono::Duration::minutes(10))
                .timestamp()
                .try_into()
                .unwrap(),
            user_id: auth_context.user_id.to_string(),
            role: auth_context.role.to_string(),
        }
    }
}
#[async_trait::async_trait]
impl AuthPacker for &JwtAuth {
    async fn pack_auth(&self, auth: AuthContext) -> String {
        let claims = Claims::new(auth);
        let header = jsonwebtoken::Header::default();
        let encoding_key = jsonwebtoken::EncodingKey::from_secret(self.secret.as_ref());
        jsonwebtoken::encode(&header, &claims, &encoding_key).unwrap()
    }
}
#[async_trait::async_trait]
impl AuthExtractor for &JwtAuth {
    async fn extract_auth(&self, input: String) -> Option<AuthContext> {
        let decoding_key = jsonwebtoken::DecodingKey::from_secret(self.secret.as_ref());
        let token_data = jsonwebtoken::decode::<Claims>(
            &input,
            &decoding_key,
            &jsonwebtoken::Validation::default(),
        )
        .ok()?;
        let claims = token_data.claims;
        let user_id = Uuid::from_str(&claims.user_id)
            .ok()
            .map(ca_domain::entity::user::Id::from)?;
        let role = Role::from_str(&claims.role).ok()?;
        Some(AuthContext { user_id, role })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    /// This test is ignored because it requires a secret key to run.
    async fn generate_admin() {
        let jwt_auth = JwtAuth::new("secret".to_string());
        let auth_context = AuthContext {
            user_id: ca_domain::entity::user::Id::new(uuid::Uuid::from_u128(0)),
            role: Role::Admin,
        };
        let token = (&jwt_auth).pack_auth(auth_context).await;
        println!("token: {}", token);
    }
    #[tokio::test]
    async fn test_exp() {
        let jwt_auth = JwtAuth::new("secret".to_string());
        let auth_context = AuthContext {
            user_id: ca_domain::entity::user::Id::new(uuid::Uuid::from_u128(0)),
            role: Role::Admin,
        };
        let token = (&jwt_auth).pack_auth(auth_context).await;
        let decoded = (&jwt_auth).extract_auth(token.clone()).await;
        assert!(decoded.is_some());
    }
}
