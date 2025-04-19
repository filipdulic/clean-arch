use crate::{
    gateway::{
        database::{
            user::{GetError, Repo, SaveError},
            Database,
        },
        DatabaseProvider,
    },
    usecase::{
        user::validate::{self, validate_user_properties, UserInvalidity},
        Usecase,
    },
};
use ca_domain::{
    entity::{
        auth_context::{AuthContext, AuthError},
        user::{Email, Id, UserName},
    },
    value_object::Password,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub id: Id,
    pub email: String,
    pub username: String,
    pub password: String,
}

pub type Response = ();

pub struct Update<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error, Serialize)]
pub enum Error {
    #[error("User {0} not found")]
    NotFound(Id),
    #[error(transparent)]
    Invalidity(#[from] UserInvalidity),
    #[error("{}", SaveError::Connection)]
    Repo,
}

impl From<SaveError> for Error {
    fn from(err: SaveError) -> Self {
        match err {
            SaveError::Connection => Self::Repo,
        }
    }
}

impl From<(GetError, Id)> for Error {
    fn from((err, id): (GetError, Id)) -> Self {
        match err {
            GetError::NotFound => Self::NotFound(id),
            GetError::Connection => Self::Repo,
        }
    }
}

impl<'d, D> Usecase<'d, D> for Update<'d, D>
where
    D: DatabaseProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    async fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Update User: {:?}", req);
        validate_user_properties(&validate::Request {
            username: &req.username,
            email: &req.email,
            password: &req.password,
        })?;
        let mut record = self
            .dependency_provider
            .database()
            .user_repo()
            .get(None, req.id)
            .await
            .map_err(|err| (err, req.id))?;
        record.user.update(
            Email::new(&req.email),
            UserName::new(&req.username),
            Password::new(&req.password),
        );
        self.dependency_provider
            .database()
            .user_repo()
            .save(None, record)
            .await?;
        Ok(())
    }

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn authorize(req: &Self::Request, auth_context: Option<AuthContext>) -> Result<(), AuthError> {
        // owner and admin only
        if let Some(auth_context) = auth_context {
            // admin allowed
            if auth_context.is_admin() {
                return Ok(());
            } else {
                // if requested id is same as auth_context id
                if &req.id == auth_context.user_id() {
                    return Ok(());
                }
            }
        }
        Err(AuthError::Unauthorized)
    }
}
