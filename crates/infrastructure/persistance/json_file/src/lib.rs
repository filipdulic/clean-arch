use ca_application::identifier::NewIdError;
use jfs::{Config, Store};
use std::{fs, io, path::Path};
use uuid::Uuid;
mod models;
mod signup_process;
mod token;
mod user;
pub mod utils;

#[derive(Debug, Clone)]
pub struct JsonFile {
    signup_processes: Store,
    users: Store,
    tokens: Store,
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
        let tokens = Store::new_with_cfg(dir.join("tokens"), cfg)?;
        Ok(Self {
            signup_processes,
            users,
            tokens,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    mod signup_process {
        use super::*;
        use ca_domain::entity::{
            signup_process::{
                EmailVerified, Id as SignupProcessId, Initialized, SignupProcess,
                VerificationEmailSent,
            },
            user::{Email, Password, UserName},
        };
        use rstest::*;
        use tempfile::TempDir;

        #[fixture]
        pub fn username() -> UserName {
            UserName::new("test_username".to_string())
        }
        #[fixture]
        pub fn password() -> Password {
            Password::new("test_pass".to_string())
        }
        #[fixture]
        pub fn id() -> SignupProcessId {
            SignupProcessId::new(Uuid::new_v4())
        }
        #[fixture]
        pub fn email() -> Email {
            Email::new("test_email".to_string())
        }
        #[rstest]
        fn save_load_signup_process(
            email: Email,
            id: SignupProcessId,
            username: UserName,
            password: Password,
        ) {
            use ca_application::gateway::repository::signup_process::{
                Record as SignupProcessRecord, Repo as SignupProcessRepo,
            };
            // -- setup --
            init();
            let test_dir = TempDir::new().unwrap();
            log::debug!("Test directory: {}", test_dir.path().display());
            let db = JsonFile::try_new(&test_dir).unwrap();
            let signup_process = SignupProcess::new(id, email);
            let record = SignupProcessRecord::from(signup_process.clone());
            (&db as &dyn SignupProcessRepo)
                .save_latest_state(record.clone())
                .unwrap();

            let db_record = (&db as &dyn SignupProcessRepo)
                .get_latest_state(id)
                .unwrap();
            assert_eq!(db_record, record);

            let signup_process = SignupProcess::<Initialized>::try_from(db_record)
                .unwrap()
                .send_verification_email();
            let record = SignupProcessRecord::from(signup_process.clone());
            (&db as &dyn SignupProcessRepo)
                .save_latest_state(record.clone())
                .unwrap();
            let db_record = (&db as &dyn SignupProcessRepo)
                .get_latest_state(id)
                .unwrap();
            // assert loaded state is the changed EmailVerified state.
            assert_eq!(db_record, record);
            // Send Email verification
            let signup_process = SignupProcess::<VerificationEmailSent>::try_from(db_record)
                .unwrap()
                .verify_email();
            // assert state has changed to Completed.
            let record = SignupProcessRecord::from(signup_process.clone());

            (&db as &dyn SignupProcessRepo)
                .save_latest_state(record.clone())
                .unwrap();

            let db_record = (&db as &dyn SignupProcessRepo)
                .get_latest_state(id)
                .unwrap();
            // assert the loaded state is the new Completed state.
            assert_eq!(db_record, record);
            // Completed step
            let signup_process = SignupProcess::<EmailVerified>::try_from(db_record)
                .unwrap()
                .complete(username, password);
            // assert state has changed to Completed.
            let record = SignupProcessRecord::from(signup_process.clone());

            (&db as &dyn SignupProcessRepo)
                .save_latest_state(record.clone())
                .unwrap();

            let db_record = (&db as &dyn SignupProcessRepo)
                .get_latest_state(id)
                .unwrap();
            // assert the loaded state is the new Completed state.
            assert_eq!(db_record, record);
        }
    }
}
