use std::{marker::PhantomData, sync::Arc};

use ca_application::usecase::{Comitable, Usecase};

use super::{
    boundary::{Error, Ingester, Presenter},
    dependency_provider::Transactional,
};

#[derive(Clone)]
pub struct Controller<D, B> {
    dependency_provider: Arc<D>,
    phantom: PhantomData<(B, D)>,
}

impl<'d, D, B> Controller<D, B>
where
    D: 'd + Transactional,
{
    pub const fn new(dependency_provider: Arc<D>) -> Self {
        Self {
            dependency_provider,
            phantom: PhantomData,
        }
    }
    pub fn handle_usecase<U>(
        &'d self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        Result<<U as Usecase<'d, D>>::Response, <U as Usecase<'d, D>>::Error>:
            Into<Comitable<<U as Usecase<'d, D>>::Response, <U as Usecase<'d, D>>::Error>>,
        U: Usecase<'d, D>,
        B: Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        let req = <B as Ingester<D, U>>::ingest(input).and_then(|req| {
            self.dependency_provider
                .run_in_transaction(|db| U::new(db).exec(req))
                .map_err(|err| Error::UsecaseError(err))
        });
        <B as Presenter<D, U>>::present(req)
    }
}
