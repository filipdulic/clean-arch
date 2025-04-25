use ca_domain::entity::auth_context::AuthContext;

#[async_trait::async_trait]
pub trait AuthExtractor {
    async fn extract_auth(&self, auth_input: String) -> Option<AuthContext>;
}

#[async_trait::async_trait]
pub trait AuthPacker {
    async fn pack_auth(&self, auth: AuthContext) -> String;
}

#[cfg(test)]
pub mod mock {
    use mockall::mock;

    mock! {
        pub AuthPacker {}
        #[async_trait::async_trait]
        impl super::AuthPacker for AuthPacker {
            async fn pack_auth(
                &self,
                auth: super::AuthContext,
            ) -> String;
        }
    }
    #[async_trait::async_trait]
    impl super::AuthPacker for &MockAuthPacker {
        async fn pack_auth(&self, auth: super::AuthContext) -> String {
            (*self).pack_auth(auth).await
        }
    }
    mock! {
        pub AuthExtractor {}
        #[async_trait::async_trait]
        impl super::AuthExtractor for AuthExtractor {
            async fn extract_auth(
                &self,
                auth_input: String,
            ) -> Option<super::AuthContext>;
        }
    }
    #[async_trait::async_trait]
    impl super::AuthExtractor for &MockAuthExtractor {
        async fn extract_auth(&self, auth_input: String) -> Option<super::AuthContext> {
            (*self).extract_auth(auth_input).await
        }
    }
}
