use super::{models, JsonFile};
use crate::application::{
    gateway::repository::signup_process::{DeleteError, GetError, Record, Repo, SaveError},
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
    fn save(&self, record: Record) -> Result<(), SaveError> {
        log::debug!("Save area of life {:?} to JSON file", record);
        let model: models::SignupProcess = record.into();
        self.signup_processes
            .save_with_id(&model, &model.signup_process_id)
            .map_err(|_| {
                log::warn!("Unable to save signup process!");
                SaveError::Connection
            })?;
        Ok(())
    }
    fn get(&self, id: Id) -> Result<Record, GetError> {
        log::debug!("Get signup process{:?} from JSON file", id);
        let model = self
            .signup_processes
            .get::<models::SignupProcess>(&id.to_string())
            .map_err(|err| {
                log::warn!("Unable to fetch thought: {}", err);
                if err.kind() == io::ErrorKind::NotFound {
                    GetError::NotFound
                } else {
                    GetError::Connection
                }
            })?;
        let record = models::SignupProcess::try_into(model).unwrap();
        Ok(record)
    }
    fn delete(&self, id: Id) -> Result<(), DeleteError> {
        log::debug!("Delete area of life {:?} from JSON file", id);
        todo!()
    }
}
