use async_trait::async_trait;
use ca_domain::entity::auth_context::AuthContext;
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthExtractor: Send + Sync {
    async fn extract_auth(&self, auth_input: String) -> Option<AuthContext>;
}

#[cfg_attr(test, automock)]
#[async_trait]
pub trait AuthPacker: Send + Sync {
    async fn pack_auth(&self, auth: AuthContext) -> String;
}

#[cfg(test)]
#[async_trait]
impl AuthPacker for &MockAuthPacker {
    async fn pack_auth(&self, auth: AuthContext) -> String {
        (*self).pack_auth(auth).await
    }
}
#[cfg(test)]
#[async_trait]
impl AuthExtractor for &MockAuthExtractor {
    async fn extract_auth(&self, auth_input: String) -> Option<AuthContext> {
        (*self).extract_auth(auth_input).await
    }
}
