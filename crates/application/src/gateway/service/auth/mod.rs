use std::future::Future;

use ca_domain::entity::auth_context::AuthContext;

pub trait AuthExtractor {
    fn extract_auth(&self, auth_input: String) -> impl Future<Output = Option<AuthContext>>;
}

pub trait AuthPacker {
    fn pack_auth(&self, auth: AuthContext) -> impl Future<Output = String>;
}
