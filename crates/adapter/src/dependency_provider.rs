use ca_application::usecase::Comitable;

pub trait Transactional: Clone {
    fn run_in_transaction<'d, F, R, E>(&'d self, f: F) -> Result<R, E>
    where
        Result<R, E>: Into<Comitable<R, E>>,
        F: FnOnce(&'d Self) -> Result<R, E>;
}
