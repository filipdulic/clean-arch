use super::{models, JsonFile};
use ca_application::{
    gateway::repository::user::{DeleteError, GetAllError, GetError, Record, Repo, SaveError},
    identifier::{NewId, NewIdError},
};
use ca_domain::entity::user::Id;
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
        let model: models::User = record.into();
        self.users
            .save_with_id(&model, &model.user_id)
            .map_err(|_| {
                log::warn!("Unable to save User!");
                SaveError::Connection
            })?;
        Ok(())
    }
    fn get(&self, id: Id) -> Result<Record, GetError> {
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
        match model.try_into() {
            Ok(record) => Ok(record),
            Err(_) => unreachable!(), // stored user should be valid
        }
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
                match model.try_into() {
                    Ok(record) => record,
                    Err(_) => unreachable!(), // stored user should be valid
                }
            })
            .collect();
        Ok(records)
    }
    // TODO: fix hack of iterating through all users to find matching username.
    fn get_by_username(
        &self,
        username: ca_domain::entity::user::UserName,
    ) -> Result<Record, GetError> {
        log::debug!("Get user by username {:?} from JSON file", username);
        for (_, model) in self
            .users
            .all::<models::User>()
            .map_err(|_| GetError::NotFound)?
        {
            if model.username == username.to_string() {
                match model.try_into() {
                    Ok(record) => return Ok(record),
                    Err(_) => unreachable!(), // stored user should be valid
                }
            }
        }
        Err(GetError::NotFound)
    }

    fn delete(&self, id: Id) -> Result<(), DeleteError> {
        log::debug!("Delete user {:?} from JSON file", &id);
        let string_id = id.to_string();
        self.users.delete(&string_id).map_err(|err| {
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
