use std::future::Future;

use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    auth_strategy::AuthStrategy,
    user::Id as UserId,
};

use serde::{de::DeserializeOwned, Serialize};

pub mod signup_process;
#[cfg(test)]
mod tests;
pub mod user;

/// Usecase trait
pub trait Usecase<'d, D> {
    type Request: DeserializeOwned;
    type Response: Serialize;
    type Error: std::fmt::Debug + Serialize;
    fn exec(&self, req: Self::Request)
        -> impl Future<Output = Result<Self::Response, Self::Error>>;
    fn new(db: &'d D) -> Self;
    fn extract_owner(&self, _req: &Self::Request) -> Option<UserId> {
        None
    }
    fn auth_strategy(&self) -> AuthStrategy {
        AuthStrategy::AdminOnly
    }
    #[allow(unused_variables)]
    fn authorize(
        &self,
        req: &Self::Request,
        auth_context: Option<AuthContext>,
    ) -> Result<(), AuthError> {
        match self.auth_strategy() {
            AuthStrategy::AdminOnly => {
                if let Some(auth_context) = auth_context {
                    if auth_context.is_admin() {
                        Ok(())
                    } else {
                        Err(AuthError::Unauthorized)
                    }
                } else {
                    Err(AuthError::Unauthorized)
                }
            }
            AuthStrategy::AdminAndOwnerOnly => {
                if let Some(auth_context) = auth_context {
                    // check if user is admin
                    if auth_context.is_admin() {
                        Ok(())
                    // check if user is owner
                    } else if let Some(owner) = self.extract_owner(req) {
                        if owner == auth_context.user_id {
                            Ok(())
                        } else {
                            Err(AuthError::Unauthorized)
                        }
                    } else {
                        // extract owner returned None
                        // for strategy AdminAndOwnerOnly
                        // this should not happen
                        unimplemented!("extract_owner returned None")
                    }
                // if no auth context is provided
                } else {
                    Err(AuthError::Unauthorized)
                }
            }
            AuthStrategy::Public => {
                // no auth required
                Ok(())
            }
        }
    }
}
