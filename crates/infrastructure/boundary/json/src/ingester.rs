use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
    usecase::Usecase,
};
use serde_json::Value;

use super::Boundary;
#[async_trait::async_trait]
impl<D, U: Usecase<D>> Ingester<D, U> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type InputModel = Value;
    async fn ingest(data: Self::InputModel) -> UsecaseRequestResult<D, U> {
        let data: <U as Usecase<D>>::Request =
            serde_json::from_value(data).map_err(|e| Error::ParseInputError(e.to_string()))?;
        Ok(data)
    }
}
