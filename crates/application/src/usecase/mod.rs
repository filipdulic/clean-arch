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
}

pub enum Comitable<R, E> {
    Commit(Result<R, E>),
    Rollback(Result<R, E>),
}
