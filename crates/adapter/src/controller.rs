use std::{marker::PhantomData, sync::Arc};

use ca_application::{
    gateway::{service::auth::AuthExtractor, AuthExtractorProvider},
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
    pub async fn handle_usecase<U>(
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
        // process input
        let processed_req = match <B as Ingester<D, U>>::ingest(input) {
            Err(err) => {
                return <B as Presenter<D, U>>::present(Err(err));
            }
            Ok(r) => r,
        };
        // Extract auth context from token
        let auth_context = if let Some(token) = token {
            self.dependency_provider
                .auth_extractor()
                .extract_auth(token.clone())
                .await
        } else {
            None
        };
        // Authorize request
        if U::authorize(&processed_req, auth_context).is_err() {
            return <B as Presenter<D, U>>::present(Err(Error::AuthError(AuthError::Unauthorized)));
        }
        // Execute use case in transaction if it is transactional
        let req = if U::is_transactional() {
            self.dependency_provider
                .run_in_transaction(async |db| U::new(db).exec(processed_req).await)
                .await
                .map_err(|err| Error::UsecaseError(err))
        } else {
            U::new(&self.dependency_provider)
                .exec(processed_req)
                .await
                .map_err(|err| Error::UsecaseError(err))
        };
        <B as Presenter<D, U>>::present(req)
    }
}
