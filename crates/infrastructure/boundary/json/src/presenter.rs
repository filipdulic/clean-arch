use ca_adapter::boundary::{Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
    usecase::Usecase,
};
use serde_json::{json, Value};

use super::Boundary;

impl<'d, D, U: Usecase<'d, D>> Presenter<'d, D, U> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type ViewModel = Value;
    fn present(data: UsecaseResponseResult<'d, D, U>) -> Self::ViewModel {
        json!(data)
    }
}
