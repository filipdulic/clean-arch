pub mod cli;

use thiserror::Error;

use crate::application::usecase::Usecase;

#[derive(Error, Debug)]
pub enum Error<'r, R, U: Usecase<'r, R>> {
    #[error("Unable to parse id")]
    ParseIdError,
    #[error("Unable to parse input")]
    ParseInputError,
    #[error("Usecase error")]
    UsecaseError(U::Error), // impl from thing...
}

type UsecaseResponseResult<'d, D, U: Usecase<'d, D>> =
    Result<<U as Usecase<'d, D>>::Response, Error<'d, D, U>>;

type UsecaseRequestResult<'d, D, U: Usecase<'d, D>> =
    Result<<U as Usecase<'d, D>>::Request, Error<'d, D, U>>;

pub trait Ingester<'a, A, U: Usecase<'a, A>> {
    type InputModel;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<'a, A, U>;
}
pub trait Presenter<'a, A, U: Usecase<'a, A>> {
    type ViewModel;
    fn present(data: UsecaseResponseResult<'a, A, U>) -> Self::ViewModel;
}
