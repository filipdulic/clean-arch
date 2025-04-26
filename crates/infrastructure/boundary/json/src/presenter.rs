use ca_adapter::boundary::{Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
    usecase::Usecase,
};
use serde_json::{json, Value};

use super::Boundary;
#[async_trait::async_trait]
impl<D, U: Usecase<D> + 'static> Presenter<D, U> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider + 'static,
{
    type ViewModel = Value;
    async fn present(data: UsecaseResponseResult<D, U>) -> Self::ViewModel {
        json!(data)
    }
}
