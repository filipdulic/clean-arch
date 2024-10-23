id_conversion!(user);

pub mod update {
    use super::{Id, ParseIdError};
    use crate::application::usecase::user::{update as uc, validate::UserInvalidity};
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("User {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
        #[error(transparent)]
        Invalidity(#[from] UserInvalidity),
    }

    impl From<uc::Error> for Error {
        fn from(from: uc::Error) -> Self {
            match from {
                uc::Error::NotFound(id) => Self::NotFound(id.into()),
                uc::Error::Invalidity(i) => Self::Invalidity(i),
                uc::Error::Repo => Self::Repo,
            }
        }
    }
}

pub mod get_one {
    use super::ParseIdError;
    use crate::application::usecase::user::get_one as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("{}", uc::Error::NotFound)]
        NotFound,
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound => Error::NotFound,
            }
        }
    }
}

pub mod get_all {
    use crate::application::usecase::user::get_all as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod delete {
    use super::ParseIdError;
    use crate::application::usecase::user::delete as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("{}", uc::Error::NotFound)]
        NotFound,
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound => Error::NotFound,
            }
        }
    }
}
