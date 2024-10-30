//! Controllers
//!
//! *"The **controller** takes user input, converts it into the request model
//! defined by the use case interactor and passes this to the same."*
//!
//! [...]
//!
//! *"It is the role of the controller to convert the given information
//! into a format which is most convenient for and defined
//! by the use case interactor."* [^1]
//!
//! [^1]: <https://www.plainionist.net/Implementing-Clean-Architecture-Controller-Presenter/>
//!
//! The controller module in the adapter layer is responsible for handling user
//! input and converting it into a format that the use case interactor can
//! understand. This aligns with the Controller role in Clean Architecture, which
//! acts as an intermediary between the user interface and the application logic.
//!
//! Key Responsibilities:
//! *Input Handling: The controller takes user input (e.g., HTTP requests,
//!     CLI commands) and converts it into request models defined by the use case
//!     interactors.
//! *Validation: It may perform basic validation on the input data before passing
//!     it to the use case.
//! *Interfacing with Use Cases: The controller calls the appropriate use case
//!     interactor methods to perform the requested actions.
//! *Error Handling: It handles any errors that occur during the interaction with
//!     the use case and prepares appropriate responses.

use std::sync::Arc;

use crate::application::usecase::Usecase;

use super::boundary::{Error, Ingester, Presenter};

pub mod signup_process;
pub mod user;

pub trait Controller<D, B> {
    fn new(db: Arc<D>, boundary: B) -> Self;
    fn db(&self) -> Arc<D>;
    fn handle_usecase<U>(
        &self,
        input: <B as Ingester<D, U>>::InputModel,
    ) -> <B as Presenter<D, U>>::ViewModel
    where
        U: Usecase<D>,
        B: Ingester<D, U> + Presenter<D, U>,
    {
        let req = <B as Ingester<D, U>>::ingest(input).and_then(|req| {
            U::new(self.db())
                .exec(req)
                .map_err(|e| Error::UsecaseError(e))
        });
        <B as Presenter<D, U>>::present(req)
    }
}
