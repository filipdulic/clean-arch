use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    user::Id as UserId,
};

/// Proof that a request passed its usecase's authorization policy.
///
/// Only a [`Policy`] can construct this, so a `Usecase::exec` taking
/// `Authorized<Request>` cannot be reached without authorization.
#[derive(Debug)]
pub struct Authorized<R> {
    request: R,
    context: Option<AuthContext>,
}

impl<R> Authorized<R> {
    pub fn request(&self) -> &R {
        &self.request
    }
    pub fn into_request(self) -> R {
        self.request
    }
    /// The authenticated caller, when the policy required one.
    pub fn context(&self) -> Option<&AuthContext> {
        self.context.as_ref()
    }
    #[cfg(test)]
    pub(crate) fn for_test(request: R) -> Self {
        Self {
            request,
            context: None,
        }
    }
}

/// An authorization policy over a usecase request.
pub trait Policy<R>: Send + Sync {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R>, AuthError>;
}

/// Ownership of a request, for [`AdminOrOwner`].
pub trait HasOwner {
    fn owner(&self) -> UserId;
}

/// No authentication required.
pub struct Public;

/// Only admins may execute.
pub struct AdminOnly;

/// Admins and the owner named by the request may execute.
pub struct AdminOrOwner;

impl<R> Policy<R> for Public {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R>, AuthError> {
        Ok(Authorized { request, context })
    }
}

impl<R> Policy<R> for AdminOnly {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R>, AuthError> {
        match context {
            Some(context) if context.is_admin() => Ok(Authorized {
                request,
                context: Some(context),
            }),
            _ => Err(AuthError::Unauthorized),
        }
    }
}

impl<R: HasOwner> Policy<R> for AdminOrOwner {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R>, AuthError> {
        match context {
            Some(context) if context.is_admin() || request.owner() == context.user_id => {
                Ok(Authorized {
                    request,
                    context: Some(context),
                })
            }
            _ => Err(AuthError::Unauthorized),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ca_domain::value_object::Role;

    #[derive(Debug)]
    struct Req {
        owner: UserId,
    }
    impl HasOwner for Req {
        fn owner(&self) -> UserId {
            self.owner
        }
    }

    fn id(n: u128) -> UserId {
        UserId::new(uuid::Uuid::from_u128(n))
    }
    fn admin() -> AuthContext {
        AuthContext::new(id(1), Role::Admin)
    }
    fn user(n: u128) -> AuthContext {
        AuthContext::new(id(n), Role::User)
    }

    #[test]
    fn public_allows_anyone() {
        assert!(Public::check(Req { owner: id(2) }, None).is_ok());
        assert!(Public::check(Req { owner: id(2) }, Some(user(3))).is_ok());
    }

    #[test]
    fn admin_only_allows_only_admins() {
        assert!(AdminOnly::check(Req { owner: id(2) }, Some(admin())).is_ok());
        assert_eq!(
            AdminOnly::check(Req { owner: id(2) }, Some(user(2))).unwrap_err(),
            AuthError::Unauthorized
        );
        assert_eq!(
            AdminOnly::check(Req { owner: id(2) }, None).unwrap_err(),
            AuthError::Unauthorized
        );
    }

    #[test]
    fn admin_or_owner_allows_admins_and_the_owner() {
        assert!(AdminOrOwner::check(Req { owner: id(2) }, Some(admin())).is_ok());
        assert!(AdminOrOwner::check(Req { owner: id(2) }, Some(user(2))).is_ok());
        assert_eq!(
            AdminOrOwner::check(Req { owner: id(2) }, Some(user(3))).unwrap_err(),
            AuthError::Unauthorized
        );
        assert_eq!(
            AdminOrOwner::check(Req { owner: id(2) }, None).unwrap_err(),
            AuthError::Unauthorized
        );
    }

    #[test]
    fn authorized_exposes_request_and_context() {
        let authorized = AdminOnly::check(Req { owner: id(2) }, Some(admin())).unwrap();
        assert_eq!(authorized.request().owner, id(2));
        assert!(authorized.context().unwrap().is_admin());
        assert_eq!(authorized.into_request().owner, id(2));
    }
}
