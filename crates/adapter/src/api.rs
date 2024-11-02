use std::{marker::PhantomData, sync::Arc};

use ca_application::usecase::Usecase;

use super::{
    boundary::{Error, Ingester, Presenter},
    db::Transactional,
};

#[derive(Clone)]
pub struct Api<D, B> {
    db: Arc<D>, // TODO: Change to Arc<<D> to allow for concurrent access
    phantom: PhantomData<B>,
}

impl<'d, D, B> Api<D, B>
where
    D: Transactional + Clone + 'd,
{
    pub const fn new(db: Arc<D>) -> Self {
        Self {
            db,
            phantom: PhantomData,
        }
    }
    pub fn handle_endpoint<U>(
        &'d self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        U: Usecase<'d, D>,
        B: Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        let req = <B as Ingester<D, U>>::ingest(input).and_then(|req| {
            self.db
                .run_in_transaction(|db| U::new(db).exec(req).map_err(Error::UsecaseError))
        });
        <B as Presenter<D, U>>::present(req)
    }
}
