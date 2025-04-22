use std::future::Future;

use ca_domain::entity::auth_context::AuthContext;

pub trait AuthExtractor {
    fn extract_auth(&self, auth_input: String) -> impl Future<Output = Option<AuthContext>>;
}

pub trait AuthPacker {
    fn pack_auth(&self, auth: AuthContext) -> impl Future<Output = String>;
}

#[cfg(test)]
pub mod mock {
    use mockall::mock;

    mock! {
        pub AuthPacker {}
        impl super::AuthPacker for AuthPacker {
            async fn pack_auth(
                &self,
                auth: super::AuthContext,
            ) -> String;
        }
    }
    impl super::AuthPacker for &MockAuthPacker {
        async fn pack_auth(&self, auth: super::AuthContext) -> String {
            (*self).pack_auth(auth).await
        }
    }
    mock! {
        pub AuthExtractor {}
        impl super::AuthExtractor for AuthExtractor {
            async fn extract_auth(
                &self,
                auth_input: String,
            ) -> Option<super::AuthContext>;
        }
    }
    impl super::AuthExtractor for &MockAuthExtractor {
        async fn extract_auth(&self, auth_input: String) -> Option<super::AuthContext> {
            (*self).extract_auth(auth_input).await
        }
    }
}
