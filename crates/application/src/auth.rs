use std::marker::PhantomData;

use ca_domain::entity::{
    auth_context::{AuthContext, AuthError},
    user::Id as UserId,
};

/// Proof that a request passed the authorization policy `P`.
///
/// Only [`Policy::check`] can construct this, and the proof carries the
/// policy that issued it, so a `Usecase::exec` taking
/// `Authorized<Request, Self::Auth>` cannot be reached without running
/// that exact policy. A proof minted by a different policy is a type
/// error:
///
/// ```compile_fail,E0308
/// use ca_application::auth::{AdminOnly, Authorized, Policy, Public};
/// struct Req;
/// let proof: Authorized<Req, AdminOnly> = Public::check(Req, None).unwrap();
/// ```
#[derive(Debug)]
pub struct Authorized<R, P> {
    request: R,
    context: Option<AuthContext>,
    _policy: PhantomData<fn() -> P>,
}

impl<R, P> Authorized<R, P> {
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
            _policy: PhantomData,
        }
    }
}

/// An authorization policy over a usecase request.
pub trait Policy<R>: Send + Sync + Sized {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R, Self>, AuthError>;
}

/// Ownership of a request, for [`AdminOrOwner`].
pub trait HasOwner {
    fn owner(&self) -> UserId;
}

/// No authentication required.
#[derive(Debug)]
pub struct Public;

/// Only admins may execute.
#[derive(Debug)]
pub struct AdminOnly;

/// Admins and the owner named by the request may execute.
#[derive(Debug)]
pub struct AdminOrOwner;

impl<R> Policy<R> for Public {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R, Self>, AuthError> {
        Ok(Authorized {
            request,
            context,
            _policy: PhantomData,
        })
    }
}

impl<R> Policy<R> for AdminOnly {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R, Self>, AuthError> {
        match context {
            Some(context) if context.is_admin() => Ok(Authorized {
                request,
                context: Some(context),
                _policy: PhantomData,
            }),
            _ => Err(AuthError::Unauthorized),
        }
    }
}

impl<R: HasOwner> Policy<R> for AdminOrOwner {
    fn check(request: R, context: Option<AuthContext>) -> Result<Authorized<R, Self>, AuthError> {
        match context {
            Some(context) if context.is_admin() || request.owner() == context.user_id => {
                Ok(Authorized {
                    request,
                    context: Some(context),
                    _policy: PhantomData,
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
