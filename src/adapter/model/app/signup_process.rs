use std::{fmt, str::FromStr};

use thiserror::Error;
use uuid::Uuid;

use crate::domain::entity::signup_process;

/// This is the public ID of an area of life.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Id(Uuid);

impl Id {
    #[must_use]
    pub const fn to_uuid(self) -> Uuid {
        self.0
    }
}

impl From<signup_process::Id> for Id {
    fn from(id: signup_process::Id) -> Self {
        Self(id.into())
    }
}

impl From<Id> for signup_process::Id {
    fn from(id: Id) -> Self {
        Self::new(id.0)
    }
}

#[derive(Debug, Error)]
#[error("Unable to parse area of life ID")]
pub struct ParseIdError;

impl FromStr for Id {
    type Err = ParseIdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let id = s.parse().map_err(|_| ParseIdError)?;
        Ok(Self(id))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub mod initialize {
    use super::ParseIdError;
    use crate::application::usecase::signup_process::initialize as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }

    impl From<uc::Error> for Error {
        fn from(from: uc::Error) -> Self {
            match from {
                uc::Error::NewId => Self::Id,
                uc::Error::Repo => Self::Repo,
            }
        }
    }
}

pub mod add_email {
    use crate::application::usecase::signup_process::add_email as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}

pub mod complete {
    use crate::application::usecase::signup_process::complete as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;
    pub type Error = uc::Error;
}
