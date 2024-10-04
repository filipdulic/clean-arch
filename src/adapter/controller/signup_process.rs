use crate::{
    adapter::{model::app::signup_process as app, presenter::Present},
    application::{
        gateway::repository::signup_process::Repo, identifier::NewId, usecase::signup_process as uc,
    },
    domain::entity::signup_process,
};

pub struct Controller<'d, 'p, D, P> {
    db: &'d D,
    presenter: &'p P,
}

impl<'d, 'p, D, P> Controller<'d, 'p, D, P>
where
    D: Repo + NewId<signup_process::Id>,
    P: Present<app::initialize::Result>
        + Present<app::add_email::Result>
        + Present<app::complete::Result>,
{
    pub const fn new(db: &'d D, presenter: &'p P) -> Self {
        Self { db, presenter }
    }
    pub fn initialize_signup_process(
        &self,
        username: impl Into<String>,
    ) -> <P as Present<app::initialize::Result>>::ViewModel {
        let username = username.into();
        log::debug!("Initializing SignupProcess for '{}'", username);
        let req = app::initialize::Request { username };
        let interactor = uc::initialize::Initialize::new(self.db, self.db);
        let res = interactor.exec(req);
        self.presenter.present(res)
    }
}
