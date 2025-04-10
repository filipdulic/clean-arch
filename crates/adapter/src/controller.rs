use std::{marker::PhantomData, sync::Arc};

use ca_application::{
    gateway::AuthExtractorProvider,
    usecase::{Comitable, Usecase},
};
use ca_domain::entity::auth_context::AuthError;

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
    D: 'd + Transactional + AuthExtractorProvider,
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
        token: Option<String>,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        Result<<U as Usecase<'d, D>>::Response, <U as Usecase<'d, D>>::Error>:
            Into<Comitable<<U as Usecase<'d, D>>::Response, <U as Usecase<'d, D>>::Error>>,
        U: Usecase<'d, D>,
        B: Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        // use authextractor to extract the auth from token. optional tokan
        let req = <B as Ingester<D, U>>::ingest(input).and_then(|req_inner| {
            // Extract auth
            let auth_context = token.as_ref().and_then(|token| {
                self.dependency_provider
                    .auth_extractor()
                    .extract_auth(token.clone())
            });
            // Auth check
            if U::authorize(&req_inner, auth_context).is_err() {
                return Err(Error::AuthError(AuthError::Unauthorized));
            }
            if U::is_transactional() {
                self.dependency_provider
                    .run_in_transaction(|db| U::new(db).exec(req_inner))
                    .map_err(|err| Error::UsecaseError(err))
            } else {
                U::new(&self.dependency_provider)
                    .exec(req_inner)
                    .map_err(|err| Error::UsecaseError(err))
            }
        });
        <B as Presenter<D, U>>::present(req)
    }
}
