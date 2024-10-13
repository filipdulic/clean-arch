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

use crate::{
    adapter::{
        controller,
        model::app::{signup_process, user},
        presenter::Present,
    },
    application::{gateway::repository as repo, identifier::NewId},
    domain,
};
use std::sync::Arc;

pub struct Api<D, P> {
    db: Arc<D>,
    presenter: P,
}

impl<D, P> Clone for Api<D, P>
where
    P: Clone,
{
    fn clone(&self) -> Self {
        let db = Arc::clone(&self.db);
        let presenter = self.presenter.clone();
        Self { db, presenter }
    }
}

impl<D, P> Api<D, P>
where
    D: repo::user::Repo
        + repo::signup_process::Repo
        + 'static
        + NewId<domain::entity::user::Id>
        + NewId<domain::entity::signup_process::Id>,
    P: Present<user::update::Result>
        + Present<user::delete::Result>
        + Present<user::get_one::Result>
        + Present<user::get_all::Result>
        + Present<signup_process::initialize::Result>
        + Present<signup_process::verify_email::Result>
        + Present<signup_process::complete::Result>,
{
    pub const fn new(db: Arc<D>, presenter: P) -> Self {
        Self { db, presenter }
    }
    fn user_controller(&self) -> controller::user::Controller<D, P> {
        controller::user::Controller::new(&self.db, &self.presenter)
    }
    fn signup_process_controller(&self) -> controller::signup_process::Controller<D, P> {
        controller::signup_process::Controller::new(&self.db, &self.presenter)
    }
    pub fn update_user(
        &self,
        id: &str,
        email: impl Into<String>,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> <P as Present<user::update::Result>>::ViewModel {
        self.user_controller()
            .update_user(id, email, username, password)
    }
    pub fn delete_user(&self, id: &str) -> <P as Present<user::delete::Result>>::ViewModel {
        self.user_controller().delete_user(id)
    }
    pub fn get_one_user(&self, id: &str) -> <P as Present<user::get_one::Result>>::ViewModel {
        self.user_controller().get_one_user(id)
    }
    pub fn read_all_users(&self) -> <P as Present<user::get_all::Result>>::ViewModel {
        self.user_controller().get_all_users()
    }
    pub fn initialize_signup_process(
        &self,
        email: impl Into<String>,
    ) -> <P as Present<signup_process::initialize::Result>>::ViewModel {
        self.signup_process_controller()
            .initialize_signup_process(email)
    }
    pub fn verify_email_to_signup_process(
        &self,
        id: &str,
    ) -> <P as Present<signup_process::verify_email::Result>>::ViewModel {
        self.signup_process_controller()
            .verify_email_to_signup_process(id)
    }
    pub fn complete_signup_process(
        &self,
        id: &str,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> <P as Present<signup_process::complete::Result>>::ViewModel {
        self.signup_process_controller()
            .complete_signup_process(id, username, password)
    }
}
