id_conversion!(signup_process);

macro_rules! pub_mod_uc_without_error_conversion {
    ($uc:ident) => {
        pub mod $uc {
            use crate::application::usecase::signup_process::$uc as uc;
            use std::result;

            pub type Request = uc::Request;
            pub type Response = uc::Response;
            pub type Result = result::Result<Response, Error>;
            pub type Error = uc::Error;
        }
    };
}

macro_rules! pub_mod_uc_with_error_conversion {
    ($uc:ident) => {
        pub mod $uc {
            use super::{Id, ParseIdError};
            use crate::application::usecase::signup_process::$uc as uc;
            use std::result;
            pub type Request = uc::Request;
            pub type Response = uc::Response;
            pub type Result = result::Result<Response, Error>;
            use thiserror::Error;

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
        }
    };
}

macro_rules! pub_mod_uc {
    // With error handling
    ($($uc:ident),+; no_error: $($skip_error_uc:ident),*) => {
        $(
            pub_mod_uc_with_error_conversion!($uc);
        )+

        $(
            pub_mod_uc_without_error_conversion!($skip_error_uc);
        )*
    };
}

// Modify the macro call to handle `initialize` separately
pub_mod_uc!(
    verify_email,
    verification_timed_out,
    extend_verification_time,
    completion_timed_out,
    extend_completion_time,
    delete,
    complete,
    get_state_chain;
    no_error: initialize
);
