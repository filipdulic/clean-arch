use super::{
    JsonFile,
    models::{self},
};
use crate::application::{
    gateway::repository::signup_process::{GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use crate::domain::entity::signup_process::Id;
use std::io;

impl NewId<Id> for JsonFile {
    fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id()?;
        Ok(id)
    }
}

impl Repo for JsonFile {
    fn save_latest_state(&self, record: Record) -> Result<(), SaveError> {
        log::debug!("Save signup process {:?} to JSON file", record);

        let model: models::SignupProcess = record.into();
        let id = model.signup_process_id.clone();
        let mut models = self
            .signup_processes
            .get::<Vec<models::SignupProcess>>(&id)
            .unwrap_or_default();
        models.push(model);
        self.signup_processes
            .save_with_id::<Vec<models::SignupProcess>>(&models, &id)
            .map_err(|_| {
                log::warn!("Unable to save signup process!");
                SaveError::Connection
            })?;
        Ok(())
    }

    fn get_latest_state(&self, id: Id) -> Result<Record, GetError> {
        log::debug!("Get signup process {:?} from JSON file", id);
        let mut models = self
            .signup_processes
            .get::<Vec<models::SignupProcess>>(&id.to_string())
            .map_err(|err| {
                log::warn!("Unable to fetch signup process: {}", err);
                if err.kind() == io::ErrorKind::NotFound {
                    GetError::NotFound
                } else {
                    GetError::Connection
                }
            })?;
        let model = models.pop().ok_or_else(|| {
            log::warn!("Signup process not found");
            GetError::NotFound
        })?;
        Ok(model.into())
    }
}
