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

impl<'d, D, B> Api<D, B>
where
    D: repo::user::Repo
        + repo::signup_process::Repo
        + NewId<domain::entity::user::Id>
        + NewId<domain::entity::signup_process::Id>
        + Clone
        + 'd,
    B: Presenter<'d, D, SpDelete<'d, D>>
        + Presenter<'d, D, Initialize<'d, D>>
        + Presenter<'d, D, VerifyEmail<'d, D>>
        + Presenter<'d, D, VerificationTimedOut<'d, D>>
        + Presenter<'d, D, ExtendVerificationTime<'d, D>>
        + Presenter<'d, D, CompletionTimedOut<'d, D>>
        + Presenter<'d, D, ExtendCompletionTime<'d, D>>
        + Presenter<'d, D, GetStateChain<'d, D>>
        + Presenter<'d, D, Complete<'d, D>>
        + Presenter<'d, D, Delete<'d, D>>
        + Presenter<'d, D, Update<'d, D>>
        + Presenter<'d, D, GetOne<'d, D>>
        + Presenter<'d, D, GetAll<'d, D>>
        + Ingester<'d, D, SpDelete<'d, D>>
        + Ingester<'d, D, Initialize<'d, D>>
        + Ingester<'d, D, VerifyEmail<'d, D>>
        + Ingester<'d, D, VerificationTimedOut<'d, D>>
        + Ingester<'d, D, ExtendVerificationTime<'d, D>>
        + Ingester<'d, D, CompletionTimedOut<'d, D>>
        + Ingester<'d, D, ExtendCompletionTime<'d, D>>
        + Ingester<'d, D, GetStateChain<'d, D>>
        + Ingester<'d, D, Complete<'d, D>>
        + Ingester<'d, D, Delete<'d, D>>
        + Ingester<'d, D, Update<'d, D>>
        + Ingester<'d, D, GetOne<'d, D>>
        + Ingester<'d, D, GetAll<'d, D>>
        + Clone,
{
    pub const fn new(db: Arc<D>, boundary: B) -> Self {
        Self { db, boundary }
    }
    fn user_controller(&'d self) -> controller::user::UserController<'d, D, B> {
        controller::user::UserController::new(&self.db, self.boundary.clone())
    }
    fn signup_process_controller(
        &'d self,
    ) -> controller::signup_process::SignupProcessController<'d, D, B> {
        controller::signup_process::SignupProcessController::new(&self.db, self.boundary.clone())
    }
    pub fn handle_user_endpont<U>(
        &'d self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        U: Usecase<'d, D>,
        B: Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        self.user_controller().handle_usecase::<U>(input)
    }
    pub fn handle_signup_process_endpoint<U>(
        &'d self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        U: Usecase<'d, D>,
        B: Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        self.signup_process_controller().handle_usecase::<U>(input)
    }
}
