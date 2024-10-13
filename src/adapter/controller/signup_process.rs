use crate::{
    adapter::{model::app::signup_process as app, presenter::Present},
    application::{
        gateway::repository::{signup_process::Repo, user::Repo as UserRepo},
        identifier::NewId,
        usecase::signup_process as uc,
    },
    domain::entity::signup_process,
};

pub struct Controller<'d, 'p, D, P> {
    db: &'d D,
    presenter: &'p P,
}

impl<'d, 'p, D, P> Controller<'d, 'p, D, P>
where
    D: Repo + NewId<signup_process::Id> + UserRepo,
    P: Present<app::initialize::Result>
        + Present<app::verify_email::Result>
        + Present<app::complete::Result>,
{
    pub const fn new(db: &'d D, presenter: &'p P) -> Self {
        Self { db, presenter }
    }
    pub fn initialize_signup_process(
        &self,
        email: impl Into<String>,
    ) -> <P as Present<app::initialize::Result>>::ViewModel {
        let email = email.into();
        log::debug!("Initializing SignupProcess for '{}'", email);
        let req = app::initialize::Request { email };
        let interactor = uc::initialize::Initialize::new(self.db, self.db);
        let res = interactor.exec(req);
        self.presenter.present(res)
    }
    pub fn verify_email_to_signup_process(
        &self,
        id: &str,
    ) -> <P as Present<app::verify_email::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::verify_email::Error::Id)
            .and_then(|id| {
                let req = app::verify_email::Request { id: id.into() };
                log::debug!("Completing SignupProcess with id: '{}'", id);
                let interactor = uc::verify_email::VerifyEmail::new(self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn complete_signup_process(
        &self,
        id: &str,
        username: impl Into<String>,
        password: impl Into<String>,
    ) -> <P as Present<app::complete::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::complete::Error::Id)
            .and_then(|id| {
                let req = app::complete::Request {
                    id: id.into(),
                    username: username.into(),
                    password: password.into(),
                };
                log::debug!("Completing SignupProcess with id: '{}'", id);
                let interactor = uc::complete::Complete::new(self.db, self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
}
