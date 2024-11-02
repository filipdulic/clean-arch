pub mod signup_process;
pub mod user;

pub trait Usecase<'d, D> {
    type Request;
    type Response;
    type Error;
    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
    fn new(db: &'d D) -> Self;
}
