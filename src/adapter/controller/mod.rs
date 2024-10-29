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

use crate::application::{
    gateway::repository as repo,
    usecase::{
        user::{delete::Delete, update::Update},
        Usecase,
    },
};

use super::boundary::{Error, Ingester, Presenter};

pub mod signup_process;
pub mod user;

pub trait Controller<'d, D: 'd, B> {
    fn db(&self) -> &'d D;
    fn handle_usecase<U>(
        &self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        U: Usecase<'d, D>,
        B: Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        let req = <B as Ingester<'d, D, U>>::ingest(input).and_then(|req| {
            U::new(self.db())
                .exec(req)
                .map_err(|e| Error::UsecaseError(e))
        });
        <B as Presenter<'d, D, U>>::present(req)
    }
}

pub struct UserController<'d, 'b, D, B> {
    db: &'d D,
    boundry: &'b B,
}

impl<'d, 'b, D, B> UserController<'d, 'b, D, B>
where
    D: repo::user::Repo,
    B: Presenter<'d, D, Delete<'d, D>>
        + Presenter<'d, D, Update<'d, D>>
        + Ingester<'d, D, Delete<'d, D>>
        + Ingester<'d, D, Update<'d, D>>,
{
    pub const fn new(db: &'d D, boundry: &'b B) -> Self {
        Self { db, boundry }
    }
}

impl<'d, 'b, D, B> Controller<'d, D, B> for UserController<'d, 'b, D, B> {
    fn db(&self) -> &'d D {
        self.db
    }
}
