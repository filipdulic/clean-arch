use crate::{
    gateway::{
        repository::{
            signup_process::{GetError, SaveError},
            user,
        },
        SignupProcessRepoProvider, UserRepoProvider,
    },
    usecase::Usecase,
};

use ca_domain::entity::{
    signup_process::{EmailVerified, Id, SignupProcess, SignupStateEnum},
    user::{Password, User, UserName},
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct Response {
    pub record: user::Record,
}
pub struct Complete<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}", SaveError::Connection)]
    Repo,
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
}

impl From<SaveError> for Error {
    fn from(e: SaveError) -> Self {
        match e {
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

impl<'d, D> Usecase<'d, D> for Complete<'d, D>
where
    D: SignupProcessRepoProvider + UserRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;

    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess Completed: {:?}", req);
        let record = self
            .dependency_provider
            .signup_process_repo()
            .get_latest_state(req.id)
            .map_err(|_| Self::Error::Repo)?;
        if let SignupStateEnum::EmailVerified { .. } = record.state {
            let process: SignupProcess<EmailVerified> =
                record.try_into().map_err(|err| (err, req.id))?;
            let username = UserName::new(req.username);
            let password = Password::new(req.password);
            let process = process.complete(username, password);
            let user: User = User::new(
                ca_domain::entity::user::Id::new(req.id),
                process.email(),
                process.username(),
                process.password(),
            );
            // Save User first, then save SignupProcess
            self.dependency_provider
                .user_repo()
                .save(user.clone().into())
                .map_err(|_| Error::Repo)?;
            // if save user fails, we should not save the signup process
            self.dependency_provider
                .signup_process_repo()
                .save_latest_state(process.clone().into())
                .map_err(|_| Self::Error::NotFound(req.id))?;
            Ok(Self::Response {
                record: user.into(),
            })
        } else {
            Err(Self::Error::Repo)
        }
    }

    fn new(db: &'d D) -> Self {
        Self {
            dependency_provider: db,
        }
    }
}
