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
        + Present<app::verification_timed_out::Result>
        + Present<app::completion_timed_out::Result>
        + Present<app::delete::Result>
        + Present<app::extend_verification_time::Result>
        + Present<app::extend_completion_time::Result>
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
    pub fn verification_timed_out(
        &self,
        id: &str,
    ) -> <P as Present<app::verification_timed_out::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::verification_timed_out::Error::Id)
            .and_then(|id| {
                let req = app::verification_timed_out::Request { id: id.into() };
                log::debug!("Verification of SignupProcess with id: '{}' timed out", id);
                let interactor = uc::verification_timed_out::VerificationTimedOut::new(self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn completion_timed_out(
        &self,
        id: &str,
    ) -> <P as Present<app::completion_timed_out::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::completion_timed_out::Error::Id)
            .and_then(|id| {
                let req = app::completion_timed_out::Request { id: id.into() };
                log::debug!("Completion of SignupProcess with id: '{}' timed out", id);
                let interactor = uc::completion_timed_out::CompletionTimedOut::new(self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn extend_verification_time(
        &self,
        id: &str,
    ) -> <P as Present<app::extend_verification_time::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::extend_verification_time::Error::Id)
            .and_then(|id| {
                let req = app::extend_verification_time::Request { id: id.into() };
                log::debug!(
                    "Extending verification time of SignupProcess with id: '{}'",
                    id
                );
                let interactor = uc::extend_verification_time::ExtendVerificationTime::new(self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn extend_completion_time(
        &self,
        id: &str,
    ) -> <P as Present<app::extend_completion_time::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::extend_completion_time::Error::Id)
            .and_then(|id| {
                let req = app::extend_completion_time::Request { id: id.into() };
                log::debug!(
                    "Extending completion time of SignupProcess with id: '{}'",
                    id
                );
                let interactor = uc::extend_completion_time::ExtendCompletionTime::new(self.db);
                interactor.exec(req).map_err(Into::into)
            });
        self.presenter.present(res)
    }
    pub fn delete(&self, id: &str) -> <P as Present<app::delete::Result>>::ViewModel {
        let res = id
            .parse::<app::Id>()
            .map_err(|_| app::delete::Error::Id)
            .and_then(|id| {
                let req = app::delete::Request { id: id.into() };
                log::debug!("Deleting SignupProcess with id: '{}'", id);
                let interactor = uc::delete::Delete::new(self.db);
                interactor.exec(req).map_err(Into::into)
            });
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
                log::debug!("Email verified of SignupProcess with id: '{}'", id);
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
