use ca_adapter::boundary::{Error, Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{
        database::signup_process::Record as SignupProcessRecord, DatabaseProvider,
        EmailVerificationServiceProvider,
    },
    usecase::{
        signup_process::{
            complete::Complete, delete::Delete, extend_completion_time::ExtendCompletionTime,
            extend_verification_time::ExtendVerificationTime, get_state_chain::GetStateChain,
            initialize::Initialize, send_verification_email::SendVerificationEmail,
            verify_email::VerifyEmail,
        },
        Usecase,
    },
};

use ca_domain::entity::signup_process::SignupStateEnum;
use chrono::{DateTime, Utc};
use poem_openapi::{payload::Json, types::ToJSON, ApiResponse, Enum, Object};

use crate::Boundary;

use super::user::UserResponse;

#[derive(Object)]
pub struct IdResponse {
    pub id: String,
}

#[derive(Enum)]
pub enum SignupStateResponseEnum {
    Initialized,
    VerificationEmailSent,
    EmailVerified,
    Completed,
    ForDeletion,
    Failed,
}

#[derive(Object)]
pub struct SignupProcessResponse {
    pub id: String,
    pub state: SignupStateResponseEnum,
    pub entered_at: DateTime<Utc>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub error: Option<String>,
}

impl From<SignupProcessRecord> for SignupProcessResponse {
    fn from(record: SignupProcessRecord) -> Self {
        match record.state {
            SignupStateEnum::Initialized { email } => Self {
                id: record.id.to_string(),
                state: SignupStateResponseEnum::Initialized,
                entered_at: record.entered_at,
                username: None,
                email: Some(email.to_string()),
                error: None,
            },
            SignupStateEnum::VerificationEmailSent { email } => Self {
                id: record.id.to_string(),
                state: SignupStateResponseEnum::VerificationEmailSent,
                entered_at: record.entered_at,
                username: None,
                email: Some(email.to_string()),
                error: None,
            },
            SignupStateEnum::EmailVerified { email } => Self {
                id: record.id.to_string(),
                state: SignupStateResponseEnum::EmailVerified,
                entered_at: record.entered_at,
                username: None,
                email: Some(email.to_string()),
                error: None,
            },
            SignupStateEnum::Completed {
                email,
                username,
                password: _,
            } => Self {
                id: record.id.to_string(),
                state: SignupStateResponseEnum::Completed,
                entered_at: record.entered_at,
                username: Some(username.to_string()),
                email: Some(email.to_string()),
                error: None,
            },
            SignupStateEnum::ForDeletion => Self {
                id: record.id.to_string(),
                state: SignupStateResponseEnum::ForDeletion,
                entered_at: record.entered_at,
                username: None,
                email: None,
                error: None,
            },
            SignupStateEnum::Failed {
                previous_state: _,
                error,
            } => Self {
                id: record.id.to_string(),
                state: SignupStateResponseEnum::Failed,
                entered_at: record.entered_at,
                username: None,
                email: None,
                error: Some(error.to_string()),
            },
        }
    }
}
#[derive(Object)]
pub struct Empty;

#[derive(ApiResponse)]
pub enum TheApiResponse<T: ToJSON> {
    /// Returns when the pet is successfully created.
    #[oai(status = 200)]
    Ok(Json<T>),
    /// Returns a bad request.
    #[oai(status = 400)]
    BadRequest(Json<String>),
    /// Returns an internal server error.
    #[oai(status = 500)]
    InternalServerError(Json<String>),
}

impl<D, U: Usecase<D>, T: ToJSON> From<Error<D, U>> for TheApiResponse<T> {
    fn from(err: Error<D, U>) -> Self {
        match err {
            Error::ParseIdError => TheApiResponse::BadRequest(Json("Invalid id".to_string())),
            Error::ParseInputError(err) => TheApiResponse::BadRequest(Json(err.to_string())),
            Error::UsecaseError(err) => {
                TheApiResponse::InternalServerError(Json(format!("Usecase error: {err:?}")))
            }
            Error::AuthError(err) => {
                TheApiResponse::InternalServerError(Json(format!("Authorization error: {err:?}")))
            }
        }
    }
}

// ========================================
// Complete Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, Complete<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<UserResponse>;

    async fn present(data: UsecaseResponseResult<D, Complete<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(UserResponse {
                id: data.record.user.id().to_string(),
                username: data.record.user.username().to_string(),
                email: data.record.user.email().to_string(),
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Delete Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, Delete<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<IdResponse>;

    async fn present(data: UsecaseResponseResult<D, Delete<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(IdResponse {
                id: data.id.to_string(),
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Extend Completion Time Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, ExtendCompletionTime<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<IdResponse>;

    async fn present(data: UsecaseResponseResult<D, ExtendCompletionTime<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(IdResponse {
                id: data.id.to_string(),
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Extend Verification Time Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, ExtendVerificationTime<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<IdResponse>;

    async fn present(data: UsecaseResponseResult<D, ExtendVerificationTime<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(IdResponse {
                id: data.id.to_string(),
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Get State Chain Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, GetStateChain<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<Vec<SignupProcessResponse>>;

    async fn present(data: UsecaseResponseResult<D, GetStateChain<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => {
                let state_chain = data
                    .state_chain
                    .into_iter()
                    .map(SignupProcessResponse::from)
                    .collect();
                TheApiResponse::Ok(Json(state_chain))
            }
            Err(err) => TheApiResponse::from(err),
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
    type ViewModel = TheApiResponse<IdResponse>;

    async fn present(data: UsecaseResponseResult<D, Initialize<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(IdResponse {
                id: data.id.to_string(),
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Send Verification Email Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, SendVerificationEmail<D>> for Boundary
where
    D: DatabaseProvider
        + EmailVerificationServiceProvider
        + std::marker::Sync
        + std::marker::Send
        + 'static,
{
    type ViewModel = TheApiResponse<IdResponse>;

    async fn present(data: UsecaseResponseResult<D, SendVerificationEmail<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(IdResponse {
                id: data.id.to_string(),
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Verify Email Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, VerifyEmail<D>> for Boundary
where
    D: DatabaseProvider
        + EmailVerificationServiceProvider
        + std::marker::Sync
        + std::marker::Send
        + 'static,
{
    type ViewModel = TheApiResponse<IdResponse>;

    async fn present(data: UsecaseResponseResult<D, VerifyEmail<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(IdResponse {
                id: data.id.to_string(),
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}
