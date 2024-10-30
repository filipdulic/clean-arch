use std::sync::Arc;

pub mod signup_process;
pub mod user;

pub trait Usecase<D> {
    type Request;
    type Response;
    type Error;
    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
    fn new(db: Arc<D>) -> Self;
}
