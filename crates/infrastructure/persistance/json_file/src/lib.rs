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
    fn new_id_inner<I>(&self) -> Result<I, NewIdError>
    where
        I: From<Uuid>,
    {
        let new_id = Uuid::new_v4();
        Ok(I::from(new_id))
    }
}
