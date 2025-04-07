use ca_adapter::boundary::{Error, Ingester, UsecaseRequestResult};
use ca_application::{
    gateway::{
        EmailVerificationServiceProvider, SignupProcessIdGenProvider, SignupProcessRepoProvider,
        TokenRepoProvider, UserRepoProvider,
    },
    usecase::Usecase,
};
use serde_json::Value;

use super::Boundary;

impl<'d, D, U: Usecase<'d, D>> Ingester<'d, D, U> for Boundary
where
    D: SignupProcessRepoProvider
        + SignupProcessIdGenProvider
        + UserRepoProvider
        + TokenRepoProvider
        + EmailVerificationServiceProvider,
{
    type InputModel = Value;
    fn ingest(data: Self::InputModel) -> UsecaseRequestResult<'d, D, U> {
        let data: <U as Usecase<'d, D>>::Request =
            serde_json::from_value(data).map_err(|e| Error::ParseInputError(e.to_string()))?;
        Ok(data)
    }
}
