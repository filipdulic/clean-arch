use ca_domain::entity::auth_context::AuthContext;
use serde::Serialize;

pub trait AuthPacker {
    type AuthOutput: Serialize;
    fn pack_auth(&self, auth: AuthContext) -> Self::AuthOutput;
}
