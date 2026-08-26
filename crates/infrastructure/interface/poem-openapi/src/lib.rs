//! Poem OpenAPI interface for the application.
//!
//! Defines HTTP endpoints and maps them to application controllers.
use std::sync::Arc;

use ca_adapter::controller::{Controller, ControllerTrait};
use ca_application::{
    gateway::{
        AuthExtractorProvider, AuthPackerProvider, DatabaseProvider,
        EmailVerificationServiceProvider,
    },
    usecase::{
        signup_process::{
            complete::Complete, delete::Delete, extend_completion_time::ExtendCompletionTime,
            extend_verification_time::ExtendVerificationTime, get_state_chain::GetStateChain,
            initialize::Initialize, send_verification_email::SendVerificationEmail,
            verify_email::VerifyEmail,
        },
        user::{
            delete::Delete as UserDelete, get_all::GetAll, get_one::GetOne, login::Login,
            update::Update,
        },
    },
};

use ca_infrastructure_boundary_poem_openapi::{
    self as boundary,
    ingester::{
        signup_process::{CompleteRequest, IdRequest, InitializeRequest, VerifyEmailRequest},
        user::{LoginRequest, UpdateRequest},
    },
    presenter::{
        signup_process::{Empty, IdResponse, SignupProcessResponse, TheApiResponse},
        user::{LoginResponse, UserResponse},
    },
};
use poem_openapi::{OpenApi, SecurityScheme, Tags, auth::Bearer, param::Path, payload::Json};

#[derive(Tags)]
enum ApiTags {
    /// Operations about user
    User,
    /// Operations about signup processes
    SignupProcess,
}

#[derive(SecurityScheme)]
#[oai(ty = "bearer", key_name = "X-Token", key_in = "header")]
struct ApiSecurityScheme(Bearer);

pub struct Api<D> {
    pub controller: Controller<D, boundary::Boundary>,
}

#[OpenApi]
impl<D> Api<D>
where
    D: DatabaseProvider
        + EmailVerificationServiceProvider
        + AuthPackerProvider
        + AuthExtractorProvider
        + 'static,
{
    pub fn new(dependency_provider: Arc<D>) -> Self {
        Self {
            controller: Controller::<D, boundary::Boundary>::new(dependency_provider),
        }
    }
    #[oai(
        path = "/signup_processes/initialize",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn initialize_signup_process(
        &self,
        request: Json<InitializeRequest>,
    ) -> TheApiResponse<IdResponse> {
        self.controller
            .handle_usecase::<Initialize<D>>(request.0, None)
            .await
    }
    #[oai(
        path = "/signup_processes/send_verification_email",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn send_verification_email_signup_process(
        &self,
        auth: ApiSecurityScheme,
        request: Json<IdRequest>,
    ) -> TheApiResponse<IdResponse> {
        self.controller
            .handle_usecase::<SendVerificationEmail<D>>(request.0, Some(auth.0.token))
            .await
    }
    #[oai(
        path = "/signup_processes/verify_email",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn verify_email_signup_process(
        &self,
        request: Json<VerifyEmailRequest>,
    ) -> TheApiResponse<IdResponse> {
        self.controller
            .handle_usecase::<VerifyEmail<D>>(request.0, None)
            .await
    }
    #[oai(
        path = "/signup_processes/extend_verification_time",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn extend_verification_time_signup_process(
        &self,
        auth: ApiSecurityScheme,
        request: Json<IdRequest>,
    ) -> TheApiResponse<IdResponse> {
        self.controller
            .handle_usecase::<ExtendVerificationTime<D>>(request.0, Some(auth.0.token))
            .await
    }
    #[oai(
        path = "/signup_processes/complete",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn complete_signup_process(
        &self,
        request: Json<CompleteRequest>,
    ) -> TheApiResponse<UserResponse> {
        self.controller
            .handle_usecase::<Complete<D>>(request.0, None)
            .await
    }
    #[oai(
        path = "/signup_processes/extend_completion_time",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn extend_completion_time_signup_process(
        &self,
        auth: ApiSecurityScheme,
        request: Json<IdRequest>,
    ) -> TheApiResponse<IdResponse> {
        self.controller
            .handle_usecase::<ExtendCompletionTime<D>>(request.0, Some(auth.0.token))
            .await
    }
    #[oai(
        path = "/signup_processes/delete",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn delete_signup_process(
        &self,
        auth: ApiSecurityScheme,
        request: Json<IdRequest>,
    ) -> TheApiResponse<IdResponse> {
        self.controller
            .handle_usecase::<Delete<D>>(request.0, Some(auth.0.token))
            .await
    }
    #[oai(
        path = "/signup_processes/get_state_chain",
        method = "post",
        tag = "ApiTags::SignupProcess"
    )]
    async fn get_state_chain_signup_process(
        &self,
        auth: ApiSecurityScheme,
        request: Json<IdRequest>,
    ) -> TheApiResponse<Vec<SignupProcessResponse>> {
        self.controller
            .handle_usecase::<GetStateChain<D>>(request.0, Some(auth.0.token))
            .await
    }
    #[oai(path = "/users/delete", method = "post", tag = "ApiTags::User")]
    async fn delete_user(
        &self,
        auth: ApiSecurityScheme,
        request: Json<IdRequest>,
    ) -> TheApiResponse<Empty> {
        self.controller
            .handle_usecase::<UserDelete<D>>(request.0, Some(auth.0.token))
            .await
    }
    #[oai(path = "/users", method = "get", tag = "ApiTags::User")]
    async fn get_all_user(&self, auth: ApiSecurityScheme) -> TheApiResponse<Vec<UserResponse>> {
        self.controller
            .handle_usecase::<GetAll<D>>((), Some(auth.0.token))
            .await
    }
    #[oai(path = "/users/:user_id", method = "get", tag = "ApiTags::User")]
    async fn get_one_user(
        &self,
        auth: ApiSecurityScheme,
        user_id: Path<String>,
    ) -> TheApiResponse<UserResponse> {
        self.controller
            .handle_usecase::<GetOne<D>>(IdRequest { id: user_id.0 }, Some(auth.0.token))
            .await
    }
    #[oai(path = "/users/login", method = "post", tag = "ApiTags::User")]
    async fn login_user(&self, request: Json<LoginRequest>) -> TheApiResponse<LoginResponse> {
        self.controller
            .handle_usecase::<Login<D>>(request.0, None)
            .await
    }
    #[oai(path = "/users/update", method = "post", tag = "ApiTags::User")]
    async fn update_user(
        &self,
        auth: ApiSecurityScheme,
        request: Json<UpdateRequest>,
    ) -> TheApiResponse<Empty> {
        self.controller
            .handle_usecase::<Update<D>>(request.0, Some(auth.0.token))
            .await
    }
}
