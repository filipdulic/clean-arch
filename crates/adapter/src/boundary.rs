use ca_application::usecase::Usecase;
use ca_domain::entity::auth_context::AuthError;
use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug, Serialize)]
pub enum Error<'d, D, U: Usecase<'d, D>> {
    #[error("Unable to parse id")]
    ParseIdError,
    #[error("Unable to parse input {0}")]
    ParseInputError(String),
    #[error("Usecase error {0:?}")]
    UsecaseError(U::Error),
    #[error("Authorization error {0}")]
    AuthError(AuthError),
}

impl<'d, D, U: Usecase<'d, D>> From<AuthError> for Error<'d, D, U> {
    fn from(err: AuthError) -> Self {
        Error::AuthError(err)
    }
}

pub type UsecaseResponseResult<'d, D, U> = Result<<U as Usecase<'d, D>>::Response, Error<'d, D, U>>;

pub type UsecaseRequestResult<'d, D, U> = Result<<U as Usecase<'d, D>>::Request, Error<'d, D, U>>;

pub trait Ingester<'d, D, U: Usecase<'d, D>> {
    type InputModel;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'d, D, U>;
}
pub trait Presenter<'d, D, U: Usecase<'d, D>> {
    type ViewModel;
    fn present(data: UsecaseResponseResult<'d, D, U>) -> Self::ViewModel;
}
