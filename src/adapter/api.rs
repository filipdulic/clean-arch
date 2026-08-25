//! API
//!
//! Single entry point for the application. Dispatches any usecase
//! generically and presents its result.

use crate::{adapter::presenter::Present, application::usecase::Usecase};
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
        Self {
            db: Arc::clone(&self.db),
            presenter: self.presenter.clone(),
        }
    }
}

impl<D, P> Api<D, P> {
    pub const fn new(db: Arc<D>, presenter: P) -> Self {
        Self { db, presenter }
    }

    pub fn handle<U: Usecase<D>>(&self, req: U::Request) -> P::ViewModel
    where
        P: Present<Result<U::Response, U::Error>>,
    {
        self.presenter.present(U::exec(&self.db, req))
    }
}
