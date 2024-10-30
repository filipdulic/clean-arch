//! API
//!
//! The api.rs file in the adapter layer serves as a facade or entry point for the
//! application's functionality. It aggregates various controllers and provides a
//! unified interface for interacting with the application's use cases. This aligns
//! with the API Gateway pattern, which simplifies the interaction with the system
//! by providing a single point of access.
//!
//! Key Responsibilities:
//! * Initialization: The Api struct initializes and holds references to the database
//!     and presenter components.
//! * Controller Aggregation: It provides methods to access different controllers
//!     (e.g., user_controller, signup_process_controller).
//! * Unified Interface: The Api struct exposes methods that correspond to various use
//!     cases, making it easier for external components (e.g., CLI, web server) to
//!     interact with the application.

use std::sync::Arc;

use crate::{
    adapter::{
        boundary::{Ingester, Presenter},
        controller,
    },
    application::{
        gateway::repository as repo,
        identifier::NewId,
        usecase::{
            signup_process::{
                complete::Complete, completion_timed_out::CompletionTimedOut,
                delete::Delete as SpDelete, extend_completion_time::ExtendCompletionTime,
                extend_verification_time::ExtendVerificationTime, get_state_chain::GetStateChain,
                initialize::Initialize, verification_timed_out::VerificationTimedOut,
                verify_email::VerifyEmail,
            },
            user::{delete::Delete, get_all::GetAll, get_one::GetOne, update::Update},
            Usecase,
        },
    },
    domain,
};

use super::controller::Controller;

#[derive(Clone)]
pub struct Api<D, B>
where
    D: Clone,
    B: Clone,
{
    db: Arc<D>, // TODO: Change to Arc<<D> to allow for concurrent access
    boundary: B,
}

impl<D, B> Api<D, B>
where
    D: repo::user::Repo
        + repo::signup_process::Repo
        + NewId<domain::entity::user::Id>
        + NewId<domain::entity::signup_process::Id>
        + Clone,
    B: Presenter<D, SpDelete<D>>
        + Presenter<D, Initialize<D>>
        + Presenter<D, VerifyEmail<D>>
        + Presenter<D, VerificationTimedOut<D>>
        + Presenter<D, ExtendVerificationTime<D>>
        + Presenter<D, CompletionTimedOut<D>>
        + Presenter<D, ExtendCompletionTime<D>>
        + Presenter<D, GetStateChain<D>>
        + Presenter<D, Complete<D>>
        + Presenter<D, Delete<D>>
        + Presenter<D, Update<D>>
        + Presenter<D, GetOne<D>>
        + Presenter<D, GetAll<D>>
        + Ingester<D, SpDelete<D>>
        + Ingester<D, Initialize<D>>
        + Ingester<D, VerifyEmail<D>>
        + Ingester<D, VerificationTimedOut<D>>
        + Ingester<D, ExtendVerificationTime<D>>
        + Ingester<D, CompletionTimedOut<D>>
        + Ingester<D, ExtendCompletionTime<D>>
        + Ingester<D, GetStateChain<D>>
        + Ingester<D, Complete<D>>
        + Ingester<D, Delete<D>>
        + Ingester<D, Update<D>>
        + Ingester<D, GetOne<D>>
        + Ingester<D, GetAll<D>>
        + Clone,
{
    pub const fn new(db: Arc<D>, boundary: B) -> Self {
        Self { db, boundary }
    }
    fn user_controller(&self) -> controller::user::UserController<D, B> {
        controller::user::UserController::new(self.db.clone(), self.boundary.clone())
    }
    fn signup_process_controller(
        &self,
    ) -> controller::signup_process::SignupProcessController<D, B> {
        controller::signup_process::SignupProcessController::new(
            self.db.clone(),
            self.boundary.clone(),
        )
    }
    pub fn handle_user_endpont<U>(
        &self,
        input: <B as Ingester<D, U>>::InputModel,
    ) -> <B as Presenter<D, U>>::ViewModel
    where
        U: Usecase<D>,
        B: Ingester<D, U> + Presenter<D, U>,
    {
        self.user_controller().handle_usecase::<U>(input)
    }
    pub fn handle_signup_process_endpoint<U>(
        &self,
        input: <B as Ingester<D, U>>::InputModel,
    ) -> <B as Presenter<D, U>>::ViewModel
    where
        U: Usecase<D>,
        B: Ingester<D, U> + Presenter<D, U>,
    {
        self.signup_process_controller().handle_usecase::<U>(input)
    }
}
