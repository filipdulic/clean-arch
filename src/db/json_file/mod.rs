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

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    mod signup_process {
        use super::*;
        use crate::domain::entity::{
            signup_process::{EmailAdded, Id as SignupProcessId, Initialized, SignupProcess},
            user::{Email, UserName},
        };
        use tempfile::TempDir;

        #[test]
        fn save_load_signup_process() {
            use crate::application::{
                gateway::repository::signup_process::{
                    Record as SignupProcessRecord, Repo as SignupProcessRepo,
                },
                identifier::NewId,
            };
            // -- setup --
            init();
            let test_dir = TempDir::new().unwrap();
            log::debug!("Test directory: {}", test_dir.path().display());
            let db = JsonFile::try_new(&test_dir).unwrap();
            let signup_process_id = (&db as &dyn NewId<SignupProcessId>).new_id().unwrap();
            let username = UserName::new("test username".to_string());
            let signup_process = SignupProcess::new(signup_process_id, username);
            let record = SignupProcessRecord::from(signup_process.clone());
            (&db as &dyn SignupProcessRepo)
                .save_latest_state(record.clone())
                .unwrap();

            let db_record = (&db as &dyn SignupProcessRepo)
                .get_latest_state(signup_process_id)
                .unwrap();
            assert_eq!(db_record, record);
            // EmailAdded step
            let signup_process = SignupProcess::<Initialized>::try_from(db_record)
                .unwrap()
                .add_email(Email::new("test@email.com".to_string()));
            let record = SignupProcessRecord::from(signup_process.clone());
            (&db as &dyn SignupProcessRepo)
                .save_latest_state(record.clone())
                .unwrap();
            let db_record = (&db as &dyn SignupProcessRepo)
                .get_latest_state(signup_process_id)
                .unwrap();
            // assert loaded state is the changed EmailAdded state.
            assert_eq!(db_record, record);
            // Completed step
            let signup_process = SignupProcess::<EmailAdded>::try_from(db_record)
                .unwrap()
                .complete();
            // assert state has changed to Completed.
            let record = SignupProcessRecord::from(signup_process.clone());

            (&db as &dyn SignupProcessRepo)
                .save_latest_state(record.clone())
                .unwrap();

            let db_record = (&db as &dyn SignupProcessRepo)
                .get_latest_state(signup_process_id)
                .unwrap();
            // assert the loaded state is the new Completed state.
            assert_eq!(db_record, record);
        }
    }
}
