use crate::{
    adapter::{model::app::user as app, presenter::Present},
    application::{gateway::repository::user::Repo, identifier::NewId, usecase::user as uc},
    domain::entity::user,
};

pub struct Controller<'d, 'p, D, P> {
    db: &'d D,
    presenter: &'p P,
}

impl<'d, 'p, D, P> Controller<'d, 'p, D, P>
where
    D: Repo + NewId<user::Id>,
    P: Present<app::delete::Result>
        + Present<app::update::Result>
        + Present<app::get_one::Result>
        + Present<app::get_all::Result>,
{
    pub const fn new(db: &'d D, presenter: &'p P) -> Self {
        Self { db, presenter }
    }
    pub fn delete_user(&self, id: &str) -> <P as Present<app::delete::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::delete::Error::Id)
            .and_then(|id| {
                let req = app::delete::Request { id: id.into() };
                log::debug!("Deleting User with id: '{}'", id);
                let interactor = uc::delete::Delete::new(self.db);
                interactor.exec(req).map_err(app::delete::Error::from)
            });
        self.presenter.present(res)
    }
    pub fn update_user(
        &self,
        id: &str,
        username: impl Into<String>,
        email: impl Into<String>,
    ) -> <P as Present<app::update::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::update::Error::Id)
            .and_then(|id| {
                let req = app::update::Request {
                    id: id.into(),
                    username: username.into(),
                    email: email.into(),
                };
                log::debug!("Updating User with id: '{}'", id);
                let interactor = uc::update::Update::new(self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn get_one_user(&self, id: &str) -> <P as Present<app::get_one::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::get_one::Error::Id)
            .and_then(|id| {
                let req = app::get_one::Request { id: id.into() };
                log::debug!("Getting User with id: '{}'", id);
                let interactor = uc::get_one::GetOne::new(self.db);
                interactor.exec(req).map_err(app::get_one::Error::from)
            });
        self.presenter.present(res)
    }
    pub fn get_all_users(&self) -> <P as Present<app::get_all::Result>>::ViewModel {
        log::debug!("Getting all Users");
        let req = app::get_all::Request {};
        let interactor = uc::get_all::GetAll::new(self.db);
        let res = interactor.exec(req).map_err(app::get_all::Error::from);
        self.presenter.present(res)
    }
}
