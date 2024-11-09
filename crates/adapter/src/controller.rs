use std::sync::Arc;

use ca_application::usecase::Usecase;

use super::dependency_provider::Transactional;

#[derive(Clone)]
pub struct Controller<D> {
    dependency_provider: Arc<D>,
}

impl<'d, D> Controller<D>
where
    D: 'd + Transactional,
{
    pub const fn new(dependency_provider: Arc<D>) -> Self {
        Self {
            dependency_provider,
        }
    }
    pub fn handle_usecase<U>(&'d self, input: U::Request) -> Result<U::Response, U::Error>
    where
        U: Usecase<'d, D>,
    {
        self.dependency_provider
            .run_in_transaction(|db| U::new(db).exec(input))
    }
}
