macro_rules! id_conversion {
    ($entity:ident) => {
        use std::{fmt, str::FromStr};

        use thiserror::Error;
        use uuid::Uuid;

        use crate::domain::entity::$entity as the_entity;

        /// This is the public ID of an signup process.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct Id(Uuid);

        impl Id {
            #[must_use]
            pub const fn to_uuid(self) -> Uuid {
                self.0
            }
        }

        impl From<the_entity::Id> for Id {
            fn from(id: the_entity::Id) -> Self {
                Self(id.into())
            }
        }

        impl From<Id> for the_entity::Id {
            fn from(id: Id) -> Self {
                Self::new(id.0)
            }
        }

        #[derive(Debug, Error)]
        #[error("Unable to parse signup process ID")]
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
    };
}

pub mod signup_process;
pub mod user;
