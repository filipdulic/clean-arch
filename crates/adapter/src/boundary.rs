use ca_application::usecase::Usecase;
use ca_domain::entity::auth_context::AuthError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum Error<D, U: Usecase<D>> {
    #[error("Unable to parse id")]
    ParseIdError,
    #[error("Unable to parse input {0}")]
    ParseInputError(String),
    #[error("Usecase error {0:?}")]
    UsecaseError(U::Error),
    #[error("Authorization error {0}")]
    AuthError(AuthError),
}

impl<D, U: Usecase<D>> From<AuthError> for Error<D, U> {
    fn from(err: AuthError) -> Self {
        Error::AuthError(err)
    }
}

pub type UsecaseResponseResult<D, U> = Result<<U as Usecase<D>>::Response, Error<D, U>>;

pub type UsecaseRequestResult<D, U> = Result<<U as Usecase<D>>::Request, Error<D, U>>;

#[async_trait::async_trait]
pub trait Ingester<D, U: Usecase<D>> {
    type InputModel: Send + Sync + 'static;
    async fn ingest(input: Self::InputModel) -> UsecaseRequestResult<D, U>;
}
#[async_trait::async_trait]
pub trait Presenter<D, U: Usecase<D>> {
    type ViewModel;
    async fn present(data: UsecaseResponseResult<D, U>) -> Self::ViewModel;
}
