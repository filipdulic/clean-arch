use crate::adapter::db::Db;
use crate::application::identifier::NewIdError;
use jfs::{Config, Store};
use std::{fs, io, path::Path};
use uuid::Uuid;
mod models;
mod signup_process;
mod user;

pub struct JsonFile {
    signup_processes: Store,
    users: Store,
}

impl JsonFile {
    pub fn try_new<P: AsRef<Path>>(dir: P) -> Result<Self, io::Error> {
        let cfg = Config {
            single: true,
            pretty: true,
            ..Default::default()
        };
        let dir = dir.as_ref();
        fs::create_dir_all(dir)?;
        let signup_processes = Store::new_with_cfg(dir.join("signup_processes"), cfg)?;
        let users = Store::new_with_cfg(dir.join("users"), cfg)?;
        Ok(Self {
            signup_processes,
            users,
        })
    }
    fn new_id<I>(&self) -> Result<I, NewIdError>
    where
        I: From<Uuid>,
    {
        let new_id = Uuid::new_v4();
        Ok(I::from(new_id))
    }
}

impl Db for JsonFile {}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     fn init() {
//         let _ = env_logger::builder().is_test(true).try_init();
//     }

//     mod area_of_life {
//         use super::*;
//         use cawr_domain::{
//             area_of_life::{AreaOfLife, Id as AolId, Name},
//             thought::{Id as ThoughtId, Thought, Title},
//         };
//         use std::collections::HashSet;
//         use tempfile::TempDir;

//         #[test]
//         fn delete_references_in_thoughts() {
//             use cawr_application::{
//                 gateway::repository::{
//                     area_of_life::{Record as AolRecord, Repo as AolRepo},
//                     thought::{Record as ThoughtRecord, Repo as ThoughtRepo},
//                 },
//                 identifier::NewId,
//             };
//             // -- setup --
//             init();
//             let test_dir = TempDir::new().unwrap();
//             log::debug!("Test directory: {}", test_dir.path().display());
//             let db = JsonFile::try_new(&test_dir).unwrap();
//             let aol_id = (&db as &dyn NewId<AolId>).new_id().unwrap();
//             let name = Name::new("test aol".to_string());
//             let area_of_life = AreaOfLife::new(aol_id, name);
//             let record = AolRecord { area_of_life };
//             (&db as &dyn AolRepo).save(record).unwrap();
//             let mut areas_of_life = HashSet::new();
//             areas_of_life.insert(aol_id);
//             let id = (&db as &dyn NewId<ThoughtId>).new_id().unwrap();
//             let thought = Thought::new(id, Title::new("foo".to_string()), areas_of_life);
//             let record = ThoughtRecord { thought };
//             (&db as &dyn ThoughtRepo).save(record).unwrap();
//             // -- test --
//             (&db as &dyn AolRepo).delete(aol_id).unwrap();
//             let rec = (&db as &dyn ThoughtRepo).get(id).unwrap();
//             assert!(rec.thought.areas_of_life().is_empty());
//         }
//     }
// }
