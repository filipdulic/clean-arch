use ca_adapter::boundary::{Error, Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{DatabaseProvider, EmailVerificationServiceProvider},
    usecase::Usecase,
};
use http::StatusCode;
use poem_openapi::payload::Response;

use super::Boundary;

impl<'d, D, U: Usecase<'d, D>> Presenter<'d, D, U> for Boundary
where
    D: DatabaseProvider + EmailVerificationServiceProvider,
{
    type ViewModel = Response<UsecaseResponseResult<'d, D, U>>;
    fn present(data: UsecaseResponseResult<'d, D, U>) -> Self::ViewModel {
        match data {
            Ok(response) => Response::new(Ok(response)).status(StatusCode::OK),
            Err(err) => match err {
                Error::ParseIdError => Response::new(Err(err)).status(StatusCode::BAD_REQUEST),
                Error::ParseInputError(_) => {
                    Response::new(Err(err)).status(StatusCode::BAD_REQUEST)
                }
                Error::UsecaseError(_) => {
                    Response::new(Err(err)).status(StatusCode::INTERNAL_SERVER_ERROR)
                }
                Error::AuthError(_) => Response::new(Err(err)).status(StatusCode::UNAUTHORIZED),
            },
        }
    }
}
