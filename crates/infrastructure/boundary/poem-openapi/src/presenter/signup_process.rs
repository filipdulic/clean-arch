use ca_adapter::boundary::{Error, Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::DatabaseProvider,
    usecase::{signup_process::initialize::Initialize, Usecase},
};
use poem_openapi::{payload::Json, ApiResponse, Object};

use crate::Boundary;

#[derive(Object)]
pub struct IdResponse {
    pub id: String,
}

#[derive(ApiResponse)]
pub enum IdApiResponse {
    /// Returns when the pet is successfully created.
    #[oai(status = 200)]
    Ok(Json<IdResponse>),
    /// Returns a bad request.
    #[oai(status = 400)]
    BadRequest(Json<String>),
    /// Returns an internal server error.
    #[oai(status = 500)]
    InternalServerError(Json<String>),
}

impl<D, U: Usecase<D>> From<Error<D, U>> for IdApiResponse {
    fn from(err: Error<D, U>) -> Self {
        match err {
            Error::ParseIdError => IdApiResponse::BadRequest(Json("Invalid id".to_string())),
            Error::ParseInputError(err) => IdApiResponse::BadRequest(Json(err.to_string())),
            Error::UsecaseError(err) => {
                IdApiResponse::InternalServerError(Json(format!("Usecase error: {err:?}")))
            }
            Error::AuthError(err) => {
                IdApiResponse::InternalServerError(Json(format!("Authorization error: {err:?}")))
            }
        }
    }
}

// ========================================
// Initialize Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, Initialize<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = IdApiResponse;

    async fn present(data: UsecaseResponseResult<D, Initialize<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => IdApiResponse::Ok(Json(IdResponse {
                id: data.id.to_string(),
            })),
            Err(err) => IdApiResponse::from(err),
        }
    }
}
