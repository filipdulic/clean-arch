use ca_domain::entity::auth_context::AuthContext;
pub trait AuthExtractor {
    type AuthInput;
    fn extract_auth(&self, input: Self::AuthInput) -> Option<AuthContext>;
}
