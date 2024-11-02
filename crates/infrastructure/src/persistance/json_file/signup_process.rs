use super::{
    models::{self},
    JsonFile,
};
use ca_application::{
    gateway::repository::signup_process::{DeleteError, GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use ca_domain::entity::signup_process::Id;
use std::io;

impl NewId<Id> for JsonFile {
    fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id()?;
        Ok(id)
    }
}

impl Repo for JsonFile {
    fn save_latest_state(&self, record: Record) -> Result<(), SaveError> {
        log::debug!("Save area of life {:?} to JSON file", record);

        let model: models::SignupProcess = record.into();
        let id = model.signup_process_id.clone();
        let res = self.signup_processes.get::<Vec<models::SignupProcess>>(&id);
        let mut models = Vec::new();
        if res.is_ok() {
            models = res.unwrap();
        }
        models.push(model);
        self.signup_processes
            .save_with_id::<Vec<models::SignupProcess>>(&models, &id)
            .map_err(|_| {
                log::warn!("Unable to save signup process!");
                SaveError::Connection
            })?;
        Ok(())
    }
    fn get_state_chain(&self, id: Id) -> Result<Vec<Record>, GetError> {
        log::debug!("Get signup process{:?} from JSON file", id);
        let models = self
            .signup_processes
            .get::<Vec<models::SignupProcess>>(&id.to_string())
            .map_err(|err| {
                log::warn!("Unable to fetch thought: {}", err);
                if err.kind() == io::ErrorKind::NotFound {
                    GetError::NotFound
                } else {
                    GetError::Connection
                }
            })?;
        let records = models.into_iter().map(|m| m.into()).collect();
        Ok(records)
    }
    fn delete(&self, id: Id) -> Result<(), DeleteError> {
        log::debug!("Delete area of life {:?} from JSON file", id);
        todo!()
    }

    fn get_latest_state(&self, id: Id) -> Result<Record, GetError> {
        log::debug!("Get signup process {:?} from JSON file", id);
        let models = self.get_state_chain(id)?;
        let model = models
            .last()
            .ok_or_else(|| {
                log::warn!("Signup process not found");
                GetError::NotFound
            })?
            .clone();
        Ok(model)
    }
}
