pub mod signup_process;
pub mod user;

/// A usecase interactor. `D` supplies the gateways the usecase needs.
pub trait Usecase<D> {
    type Request;
    type Response;
    type Error: core::error::Error;

    fn exec(db: &D, req: Self::Request) -> Result<Self::Response, Self::Error>;
}
