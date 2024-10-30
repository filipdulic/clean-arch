pub mod cli;

use thiserror::Error;

use crate::application::usecase::Usecase;

#[derive(Error, Debug)]
pub enum Error<R, U: Usecase<R>> {
    #[error("Unable to parse id")]
    ParseIdError,
    #[error("Unable to parse input")]
    ParseInputError,
    #[error("Usecase error")]
    UsecaseError(U::Error), // impl from thing...
}

type UsecaseResponseResult<D, U> = Result<<U as Usecase<D>>::Response, Error<D, U>>;

type UsecaseRequestResult<D, U> = Result<<U as Usecase<D>>::Request, Error<D, U>>;

pub trait Ingester<A, U: Usecase<A>> {
    type InputModel;
    fn ingest(input: Self::InputModel) -> UsecaseRequestResult<A, U>;
}
pub trait Presenter<A, U: Usecase<A>> {
    type ViewModel;
    fn present(data: UsecaseResponseResult<A, U>) -> Self::ViewModel;
}
