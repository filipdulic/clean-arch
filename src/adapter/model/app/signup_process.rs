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

// todo add validation
pub mod initialize {
    use crate::application::usecase::signup_process::initialize as uc;
    use std::result;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Error = uc::Error;
    pub type Result = result::Result<Response, Error>;
}

pub mod verify_email {
    use super::{Id, ParseIdError};
    use crate::application::usecase::signup_process::verify_email as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("SignupProcess {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound(id) => Error::NotFound(id.into()),
            }
        }
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }
}

pub mod verification_timed_out {
    use super::{Id, ParseIdError};
    use crate::application::usecase::signup_process::verification_timed_out as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("SignupProcess {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound(id) => Error::NotFound(id.into()),
            }
        }
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }
}

pub mod extend_verification_time {
    use super::{Id, ParseIdError};
    use crate::application::usecase::signup_process::extend_verification_time as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("SignupProcess {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound(id) => Error::NotFound(id.into()),
            }
        }
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }
}

pub mod completion_timed_out {
    use super::{Id, ParseIdError};
    use crate::application::usecase::signup_process::completion_timed_out as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("SignupProcess {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound(id) => Error::NotFound(id.into()),
            }
        }
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }
}

pub mod extend_completion_time {
    use super::{Id, ParseIdError};
    use crate::application::usecase::signup_process::extend_completion_time as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("SignupProcess {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound(id) => Error::NotFound(id.into()),
            }
        }
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }
}

pub mod delete {
    use super::{Id, ParseIdError};
    use crate::application::usecase::signup_process::delete as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("SignupProcess {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound(id) => Error::NotFound(id.into()),
            }
        }
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }
}

// TODO: add id error maping
pub mod complete {
    use super::{Id, ParseIdError};
    use crate::application::usecase::signup_process::complete as uc;
    use std::result;
    use thiserror::Error;

    pub type Request = uc::Request;
    pub type Response = uc::Response;
    pub type Result = result::Result<Response, Error>;

    #[derive(Debug, Error)]
    pub enum Error {
        #[error("{}", ParseIdError)]
        Id,
        #[error("SignupProcess {0:?} not found")]
        NotFound(Id),
        #[error("{}", uc::Error::Repo)]
        Repo,
    }

    impl From<uc::Error> for Error {
        fn from(e: uc::Error) -> Self {
            match e {
                uc::Error::Repo => Error::Repo,
                uc::Error::NotFound(id) => Error::NotFound(id.into()),
            }
        }
    }

    impl From<ParseIdError> for Error {
        fn from(_: ParseIdError) -> Self {
            Self::Id
        }
    }
}
