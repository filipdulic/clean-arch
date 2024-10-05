use super::{models, JsonFile};
use crate::application::{
    gateway::repository::user::{DeleteError, GetAllError, GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use crate::domain::entity::user::Id;
use std::io;

impl NewId<Id> for JsonFile {
    fn new_id(&self) -> Result<Id, NewIdError> {
        let id = self.new_id()?;
        Ok(id)
    }
}

impl Repo for JsonFile {
    fn save(&self, record: impl Into<Record>) -> Result<(), SaveError> {
        let record: Record = record.into();
        log::debug!("Save area of life {:?} to JSON file", record);
        let model: models::User = record.into();
        self.users
            .save_with_id(&model, &model.user_id)
            .map_err(|_| {
                log::warn!("Unable to save User!");
                SaveError::Connection
            })?;
        Ok(())
    }
    fn get(&self, id: impl Into<Id>) -> Result<Record, GetError> {
        let id: Id = id.into();
        log::debug!("Get user{:?} from JSON file", id);
        let model = self
            .users
            .get::<models::User>(&id.to_string())
            .map_err(|err| {
                log::warn!("Unable to fetch user: {}", err);
                if err.kind() == io::ErrorKind::NotFound {
                    GetError::NotFound
                } else {
                    GetError::Connection
                }
            })?;
        let record = models::User::try_into(model).unwrap();
        Ok(record)
    }
    fn get_all(&self) -> Result<Vec<Record>, GetAllError> {
        log::debug!("Get all users from JSON file");
        let binding = self.users.all::<models::User>().map_err(|err| {
            log::warn!("Unable to fetch users: {}", err);
            GetAllError::Connection
        })?;
        let models: Vec<&models::User> = binding.values().collect();
        let records = models
            .into_iter()
            .map(|model| {
                let record: Record = model.try_into().unwrap();
                record
            })
            .collect();
        Ok(records)
    }
    fn delete(&self, id: impl Into<Id>) -> Result<(), DeleteError> {
        let id = id.into().to_string();
        log::debug!("Delete user {:?} from JSON file", &id);
        self.users.delete(&id).map_err(|err| {
            log::warn!("Unable to delete user: {}", err);
            if err.kind() == io::ErrorKind::NotFound {
                DeleteError::NotFound
            } else {
                DeleteError::Connection
            }
        })?;
        Ok(())
    }
}
