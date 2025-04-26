use std::{marker::PhantomData, sync::Arc};

use ca_application::{
    gateway::{service::auth::AuthExtractor, AuthExtractorProvider},
    usecase::Usecase,
};
use ca_domain::entity::auth_context::AuthError;

use super::boundary::{Error, Ingester, Presenter};

#[derive(Clone)]
pub struct Controller<D, B> {
    dependency_provider: Arc<D>,
    phantom: PhantomData<(B, D)>,
}

#[async_trait::async_trait]
pub trait ControllerTrait<D, B>
where
    D: AuthExtractorProvider,
{
    fn dependency_provider(&self) -> Arc<D>;
    async fn handle_usecase<U>(
        &self,
        input: <B as Ingester<D, U>>::InputModel,
        token: Option<String>,
    ) -> <B as Presenter<D, U>>::ViewModel
    where
        U: Usecase<D>,
        B: Ingester<D, U> + Presenter<D, U>,
    {
        // process input
        let processed_req = match <B as Ingester<D, U>>::ingest(input).await {
            Err(err) => {
                return <B as Presenter<D, U>>::present(Err(err)).await;
            }
            Ok(r) => r,
        };
        // Extract auth context from token
        let auth_context = if let Some(token) = token {
            self.dependency_provider()
                .auth_extractor()
                .extract_auth(token.clone())
                .await
        } else {
            None
        };
        // Instantiate the usecase
        let usecase = U::new(self.dependency_provider());
        // Authorize request
        if usecase.authorize(&processed_req, auth_context).is_err() {
            return <B as Presenter<D, U>>::present(Err(Error::AuthError(AuthError::Unauthorized)))
                .await;
        }
        // Execute use case in transaction if it is transactional
        let req = usecase
            .exec(processed_req)
            .await
            .map_err(|err| Error::UsecaseError(err));
        <B as Presenter<D, U>>::present(req).await
    }
}

impl<D, B> Controller<D, B>
where
    D: AuthExtractorProvider,
{
    pub const fn new(dependency_provider: Arc<D>) -> Self {
        Self {
            dependency_provider,
            phantom: PhantomData,
        }
    }
}

impl<D, B> ControllerTrait<D, B> for Controller<D, B>
where
    D: AuthExtractorProvider,
{
    fn dependency_provider(&self) -> Arc<D> {
        self.dependency_provider.clone()
    }
}
