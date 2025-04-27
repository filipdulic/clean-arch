use ca_adapter::boundary::{Presenter, UsecaseResponseResult};
use ca_application::{
    gateway::{AuthPackerProvider, DatabaseProvider},
    usecase::user::{
        delete::Delete, get_all::GetAll, get_one::GetOne, login::Login, update::Update,
    },
};
use ca_domain::entity::user::User;
use poem_openapi::{payload::Json, Object};

use crate::Boundary;

use super::signup_process::{Empty, TheApiResponse};

#[derive(Object)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
    pub email: String,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        Self {
            id: value.id().to_string(),
            username: value.username().to_string(),
            email: value.email().to_string(),
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
    type ViewModel = TheApiResponse<Empty>;

    async fn present(data: UsecaseResponseResult<D, Delete<D>>) -> Self::ViewModel {
        match data {
            Ok(_) => TheApiResponse::Ok(Json(Empty)),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Get All Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, GetAll<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<Vec<UserResponse>>;

    async fn present(data: UsecaseResponseResult<D, GetAll<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(
                data.users.into_iter().map(UserResponse::from).collect(),
            )),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Get One Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, GetOne<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<UserResponse>;

    async fn present(data: UsecaseResponseResult<D, GetOne<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(UserResponse::from(data.user))),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Login Use Case
// ========================================

#[derive(Object)]
pub struct LoginResponse {
    id: String,
    token: String,
}

#[async_trait::async_trait]
impl<D> Presenter<D, Login<D>> for Boundary
where
    D: DatabaseProvider + AuthPackerProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<LoginResponse>;

    async fn present(data: UsecaseResponseResult<D, Login<D>>) -> Self::ViewModel {
        match data {
            Ok(data) => TheApiResponse::Ok(Json(LoginResponse {
                id: data.user_id.to_string(),
                token: data.token,
            })),
            Err(err) => TheApiResponse::from(err),
        }
    }
}

// ========================================
// Update Use Case
// ========================================
#[async_trait::async_trait]
impl<D> Presenter<D, Update<D>> for Boundary
where
    D: DatabaseProvider + std::marker::Sync + std::marker::Send + 'static,
{
    type ViewModel = TheApiResponse<Empty>;

    async fn present(data: UsecaseResponseResult<D, Update<D>>) -> Self::ViewModel {
        match data {
            Ok(_) => TheApiResponse::Ok(Json(Empty)),
            Err(err) => TheApiResponse::from(err),
        }
    }
}
