// use crate::{
//     adapter::{
//         controller,
//         model::app::{signup_process, user},
//         presenter::Present,
//     },
//     application::{gateway::repository as repo, identifier::NewId},
//     domain,
// };
// use std::{collections::HashSet, sync::Arc};

// pub struct Api<D, P> {
//     db: Arc<D>,
//     presenter: P,
// }

// impl<D, P> Clone for Api<D, P>
// where
//     P: Clone,
// {
//     fn clone(&self) -> Self {
//         let db = Arc::clone(&self.db);
//         let presenter = self.presenter.clone();
//         Self { db, presenter }
//     }
// }

// impl<D, P> Api<D, P>
// where
//     D: repo::user::Repo
//         + repo::signup_process::Repo
//         + 'static
//         + NewId<domain::entity::user::Id>
//         + NewId<domain::entity::signup_process::Id>,
//     P: Present<user::update::Result>
//         + Present<user::delete::Result>
//         + Present<user::get_one::Result>
//         + Present<user::get_all::Result>
//         + Present<signup_process::initialize::Result>
//         + Present<signup_process::add_email::Result>
//         + Present<signup_process::complete::Result>
// {
//     pub const fn new(db: Arc<D>, presenter: P) -> Self {
//         Self { db, presenter }
//     }
//     fn user_controller(&self) -> controller::user::Controller<D, P> {
//         controller::user::Controller::new(&self.db, &self.presenter)
//     }
//     fn signup_process_controller(&self) -> controller::signup_process::Controller<D, P> {
//         controller::signup_process::Controller::new(&self.db, &self.presenter)
//     }
//     pub fn create_user(
//         &self,
//         title: impl Into<String>,
//         areas_of_life: &HashSet<String>,
//     ) -> <P as Present<user::create::Result>>::ViewModel {
//         self.user_controller()
//             .create_user(title, areas_of_life)
//     }
//     pub fn update_user(
//         &self,
//         id: &str,
//         title: impl Into<String>,
//         areas_of_life: &HashSet<String>,
//     ) -> <P as Present<user::update::Result>>::ViewModel {
//         self.user_controller()
//             .update_user(id, title, areas_of_life)
//     }
//     pub fn delete_user(&self, id: &str) -> <P as Present<user::delete::Result>>::ViewModel {
//         self.user_controller().delete_user(id)
//     }
//     pub fn find_user(&self, id: &str) -> <P as Present<user::find_by_id::Result>>::ViewModel {
//         self.user_controller().find_user(id)
//     }
//     pub fn read_all_users(&self) -> <P as Present<user::read_all::Result>>::ViewModel {
//         self.user_controller().read_all_users()
//     }
//     pub fn create_signup_process(
//         &self,
//         name: impl Into<String>,
//     ) -> <P as Present<aol::create::Result>>::ViewModel {
//         self.aol_controller().create_signup_process(name)
//     }
//     pub fn update_signup_process(
//         &self,
//         id: &str,
//         name: impl Into<String>,
//     ) -> <P as Present<aol::update::Result>>::ViewModel {
//         self.aol_controller().update_signup_process(id, name)
//     }
//     pub fn delete_signup_process(&self, id: &str) -> <P as Present<aol::delete::Result>>::ViewModel {
//         self.aol_controller().delete_signup_process(id)
//     }
//     pub fn read_all_areas_of_life(&self) -> <P as Present<aol::read_all::Result>>::ViewModel {
//         self.aol_controller().read_all_areas_of_life()
//     }
// }
