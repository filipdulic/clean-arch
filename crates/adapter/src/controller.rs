use std::{marker::PhantomData, sync::Arc};

use ca_application::{
    auth::Policy,
    gateway::{AuthExtractorProvider, service::auth::AuthExtractor},
    usecase::Usecase,
};

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
        // Authorize the request; only the policy can mint an Authorized<Request>
        let authorized = match U::Auth::check(processed_req, auth_context) {
            Ok(authorized) => authorized,
            Err(err) => {
                return <B as Presenter<D, U>>::present(Err(Error::AuthError(err))).await;
            }
        };
        // Instantiate and execute the usecase
        let usecase = U::new(self.dependency_provider());
        let req = usecase
            .exec(authorized)
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
