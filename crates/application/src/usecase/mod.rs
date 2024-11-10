pub mod signup_process;
pub mod user;

pub trait Usecase<'d, D> {
    type Request;
    type Response;
    type Error: std::fmt::Debug;
    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
    fn new(db: &'d D) -> Self;
}

pub enum Comitable<R, E> {
    Commit(Result<R, E>),
    Rollback(Result<R, E>),
}
