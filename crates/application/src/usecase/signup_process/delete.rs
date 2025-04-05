use crate::{
    gateway::{
        repository::signup_process::{GetError, SaveError},
        SignupProcessRepoProvider,
    },
    usecase::{Comitable, Usecase},
};

use ca_domain::entity::signup_process::{
    EmailVerified, Failed, Id, SignupProcess, SignupStateEnum, VerificationEmailSent,
};

use thiserror::Error;

#[derive(Debug)]
pub struct Request {
    pub id: Id,
}

#[derive(Debug)]
pub struct Response {
    pub id: Id,
}
pub struct Delete<'d, D> {
    dependency_provider: &'d D,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("SignupProcess {0} not found")]
    NotFound(Id),
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

impl<'d, D> Usecase<'d, D> for Delete<'d, D>
where
    D: SignupProcessRepoProvider,
{
    type Request = Request;
    type Response = Response;
    type Error = Error;
    fn exec(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        log::debug!("SignupProcess scheduled for deletion: {:?}", req);
        let record = self
            .dependency_provider
            .signup_process_repo()
            .get_latest_state(req.id)
            .map_err(|err| (err, req.id))?;
        let process = match &record.state {
            SignupStateEnum::Failed {
                previous_state,
                error: _,
            } => match **previous_state {
                SignupStateEnum::VerificationEmailSent { .. } => {
                    SignupProcess::<Failed<VerificationEmailSent>>::try_from(record)
                        .map_err(|_| (GetError::NotFound, req.id))?
                        .delete()
                }
                SignupStateEnum::EmailVerified { .. } => {
                    SignupProcess::<Failed<EmailVerified>>::try_from(record)
                        .map_err(|_| (GetError::NotFound, req.id))?
                        .delete()
                }
                _ => return Err((GetError::NotFound, req.id).into()),
            },
            _ => return Err((GetError::NotFound, req.id).into()),
        };

        self.dependency_provider
            .signup_process_repo()
            .save_latest_state(process.into())
            .map_err(|_| Self::Error::NotFound(req.id))?;
        Ok(Self::Response { id: req.id })
    }
    fn new(dependency_provider: &'d D) -> Self {
        Self {
            dependency_provider,
        }
    }
    fn is_transactional() -> bool {
        true
    }
}

impl From<Result<Response, Error>> for Comitable<Response, Error> {
    fn from(res: Result<Response, Error>) -> Self {
        match res {
            Ok(res) => Comitable::Commit(Ok(res)),
            Err(err) => Comitable::Rollback(Err(err)),
        }
    }
}
