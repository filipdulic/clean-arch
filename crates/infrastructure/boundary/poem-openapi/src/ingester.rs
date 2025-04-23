use super::Boundary;
use ca_adapter::boundary::{Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
    usecase::Usecase,
};
use poem_openapi::payload::Json;

impl<'d, D, U: Usecase<'d, D>> Ingester<'d, D, U> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type InputModel = Json<U::Request>;
    fn ingest(data: Self::InputModel) -> UsecaseRequestResult<'d, D, U> {
        Ok(data.0)
    }
}
