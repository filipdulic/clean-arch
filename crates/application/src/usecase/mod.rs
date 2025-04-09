use ca_domain::entity::auth_context::{AuthContext, AuthError};
use serde::{de::DeserializeOwned, Serialize};

pub mod signup_process;
pub mod user;

pub trait Usecase<'d, D> {
    type Request: DeserializeOwned;
    type Response: Serialize;
    type Error: std::fmt::Debug + Serialize;
    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
    fn new(db: &'d D) -> Self;
    fn is_transactional() -> bool {
        false
    }
    #[allow(unused_variables)]
    fn authorize(req: &Self::Request, auth_context: Option<AuthContext>) -> Result<(), AuthError> {
        Err(AuthError::Unauthorized)
    }
}

pub enum Comitable<R, E> {
    Commit(Result<R, E>),
    Rollback(Result<R, E>),
}
