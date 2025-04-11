use std::{future::Future, ops::AsyncFnOnce};

use ca_application::usecase::Comitable;

pub trait Transactional: Clone {
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> impl Future<Output = Result<R, E>>
    where
        Result<R, E>: Into<Comitable<R, E>>,
        F: AsyncFnOnce(&'d Self) -> Result<R, E>;
}
