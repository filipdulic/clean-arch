use std::sync::Arc;

use crate::{
    adapter::boundary::{Ingester, Presenter},
    application::{
        gateway::repository as repo,
        usecase::user::{delete::Delete, get_all::GetAll, get_one::GetOne, update::Update},
    },
};

use super::Controller;

pub struct UserController<D, B> {
    db: Arc<D>,
    #[allow(dead_code)]
    boundry: B,
}

impl<D, B> Controller<D, B> for UserController<D, B>
where
    D: repo::user::Repo,
    B: Presenter<D, Delete<D>>
        + Presenter<D, Update<D>>
        + Presenter<D, GetOne<D>>
        + Presenter<D, GetAll<D>>
        + Ingester<D, Delete<D>>
        + Ingester<D, GetOne<D>>
        + Ingester<D, GetAll<D>>
        + Ingester<D, Update<D>>,
{
    fn new(db: Arc<D>, boundry: B) -> Self {
        Self { db, boundry }
    }

    fn db(&self) -> Arc<D> {
        self.db.clone()
    }
}
