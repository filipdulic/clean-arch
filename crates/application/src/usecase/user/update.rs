use crate::{
    gateway::{
        repository::user::{GetError, SaveError},
        UserRepoProvider,
    },
    usecase::{
        user::validate::{self, validate_user_properties, UserInvalidity},
        Usecase,
    },
};
use ca_domain::{
    entity::user::{Email, Id, User, UserName},
    value_object::Password,
};
use thiserror::Error;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug)]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Debug, Error)]
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
    D: UserRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("Update User: {:?}", req);
        validate_user_properties(&validate::Request {
            username: &req.username,
            email: &req.email,
            password: &req.password,
        })?;
        let username = UserName::new(req.username);
        let email = Email::new(&req.email);
        let password = Password::new(req.password);
        let user = User::new(req.id, email, username, password);
        let _ = self
            .dependency_provider
            .user_repo()
            .get(req.id)
            .map_err(|err| (err, req.id))?;
        self.dependency_provider.user_repo().save(user.into())?;
        Ok(())
    }

    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
}
