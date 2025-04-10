use ca_domain::entity::auth_context::AuthContext;

pub trait AuthExtractor {
    fn extract_auth(&self, auth_input: String) -> Option<AuthContext>;
}

pub trait AuthPacker {
    fn pack_auth(&self, auth: AuthContext) -> String;
}
