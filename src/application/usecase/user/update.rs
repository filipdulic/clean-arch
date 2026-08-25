use crate::{
    application::{
        gateway::repository::user::{GetError, Repo, SaveError},
        usecase::Usecase,
        usecase::user::validate::{self, UserInvalidity, validate_user_properties},
    },
    domain::{
        entity::user::{Email, Id, User, UserName},
        value_object::Password,
    },
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
    pub email: String,
    pub username: String,
    pub password: String,
}

pub type Response = ();

pub struct Update;

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

impl<D: Repo> Usecase<D> for Update {
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(db: &D, req: Request) -> Result<Response, Error> {
        log::debug!("Update User: {:?}", req);
        validate_user_properties(&validate::Request {
            username: &req.username,
            email: &req.email,
            password: &req.password,
        })?;
        let username = UserName::new(req.username);
        let email = Email::new(req.email);
        let password = Password::new(req.password);
        let user = User::new(req.id, email, username, password);
        let _ = db.get(req.id).map_err(|err| (err, req.id))?;
        db.save(user.into())?;
        Ok(())
    }
}
