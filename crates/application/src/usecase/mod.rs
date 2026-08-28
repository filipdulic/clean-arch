use std::sync::Arc;

use serde::{Serialize, de::DeserializeOwned};

use crate::auth::{Authorized, Policy};

pub mod signup_process;
#[cfg(test)]
mod tests;
pub mod user;

/// Usecase trait
#[async_trait::async_trait]
pub trait Usecase<D>: Send + Sync {
    type Request: DeserializeOwned + Send;
    type Response: Serialize + Send + 'static;
    type Error: std::fmt::Debug + Serialize + Send;
    /// The authorization policy guarding this usecase. `exec` takes an
    /// [`Authorized`] request, which only the policy can produce.
    type Auth: Policy<Self::Request>;
    async fn exec(
        &self,
        req: Authorized<Self::Request, Self::Auth>,
    ) -> Result<Self::Response, Self::Error>;
    fn new(db: Arc<D>) -> Self;
}
