use crate::{
    adapter::boundary::{Ingester, Presenter},
    application::{
        gateway::repository as repo,
        usecase::user::{delete::Delete, get_all::GetAll, get_one::GetOne, update::Update},
    },
};

use super::Controller;

pub struct UserController<'d, D, B> {
    db: &'d D,
    #[allow(dead_code)]
    boundry: B,
}

impl<'d, D, B> Controller<'d, D, B> for UserController<'d, D, B>
where
    D: repo::user::Repo + 'd,
    B: Presenter<'d, D, Delete<'d, D>>
        + Presenter<'d, D, Update<'d, D>>
        + Presenter<'d, D, GetOne<'d, D>>
        + Presenter<'d, D, GetAll<'d, D>>
        + Ingester<'d, D, Delete<'d, D>>
        + Ingester<'d, D, GetOne<'d, D>>
        + Ingester<'d, D, GetAll<'d, D>>
        + Ingester<'d, D, Update<'d, D>>,
{
    fn new(db: &'d D, boundry: B) -> Self {
        Self { db, boundry }
    }

    fn db(&self) -> &'d D {
        self.db
    }
}
