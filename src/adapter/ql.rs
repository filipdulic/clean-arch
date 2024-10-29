use crate::application::gateway::repository as repo;
use crate::application::usecase::user::{self, delete::Delete, update::Update};
use crate::application::usecase::Usecase;

use crate::adapter::boundary::{Error, Ingester, Presenter};
use crate::adapter::controller::Controller;
// pub struct Controller<'d, 'b, D, B> {
//     db: &'d D,
//     boundry: &'b B,
// }

// impl<'d, 'b, D, B> Controller<'d, 'b, D, B>
// where
//     D: repo::user::Repo,
//     B: Presenter<'d, D, Delete<'d, D>>
//         + Presenter<'d, D, Update<'d, D>>
//         + Ingester<'d, D, Delete<'d, D>>
//         + Ingester<'d, D, Update<'d, D>>,
// {
//     pub const fn new(db: &'d D, boundry: &'b B) -> Self {
//         Self { db, boundry }
//     }

//     pub fn handle_usecase<U>(
//         &self,
//         input: <B as Ingester<'d, D, U>>::InputModel,
//     ) -> <B as Presenter<'d, D, U>>::ViewModel
//     where
//         U: Usecase<'d, D>,
//         B: Ingester<'d, D, U> + Presenter<'d, D, U>,
//     {
//         let req = <B as Ingester<'d, D, U>>::ingest(input).and_then(|req| {
//             U::new(self.db)
//                 .exec(req)
//                 .map_err(|e| Error::UsecaseError(e))
//         });
//         <B as Presenter<'d, D, U>>::present(req)
//     }
// }

pub struct UserStringBoundry;
impl<'r, R> Ingester<'r, R, Delete<'r, R>> for UserStringBoundry
where
    R: repo::user::Repo,
{
    // <B as Ingester<'d, D, U>>::InputModel
    type InputModel = String;
    fn ingest(
        input: String,
    ) -> Result<<Delete<'r, R> as Usecase<'r, R>>::Request, Error<'r, R, Delete<'r, R>>> {
        uuid::Uuid::parse_str(&input)
            .map_err(|_| Error::ParseIdError)
            .map(|id| user::delete::Request { id: id.into() })
    }
}
impl<'r, R> Presenter<'r, R, Delete<'r, R>> for UserStringBoundry
where
    R: repo::user::Repo,
{
    type ViewModel = String;
    fn present(
        data: Result<<Delete<'r, R> as Usecase<'r, R>>::Response, Error<'r, R, Delete<'r, R>>>,
    ) -> Self::ViewModel {
        match data {
            Ok(_) => "Deleted User".to_string(),
            Err(_) => "Unable to delete user".to_string(),
        }
    }
}

pub struct Input {
    id: String,
    email: String,
    username: String,
    password: String,
}

impl<'r, R> Ingester<'r, R, Update<'r, R>> for UserStringBoundry
where
    R: repo::user::Repo,
{
    type InputModel = Input;
    fn ingest(
        input: Self::InputModel,
    ) -> Result<<Update<'r, R> as Usecase<'r, R>>::Request, Error<'r, R, Update<'r, R>>> {
        uuid::Uuid::parse_str(&input.id)
            .map_err(|_| Error::ParseInputError)
            .map(|id| user::update::Request {
                id: id.into(),
                email: input.email,
                username: input.username,
                password: input.password,
            })
    }
}

impl<'r, R> Presenter<'r, R, Update<'r, R>> for UserStringBoundry
where
    R: repo::user::Repo,
{
    type ViewModel = String;
    fn present(
        data: Result<<Update<'r, R> as Usecase<'r, R>>::Response, Error<'r, R, Update<'r, R>>>,
    ) -> Self::ViewModel {
        match data {
            Ok(_) => "Updated User".to_string(),
            Err(_) => "Unable to update user".to_string(),
        }
    }
}

pub struct API<'d, D, B> {
    db: &'d D,
    boundry: B,
}

impl<'d, D, B> API<'d, D, B>
where
    D: repo::user::Repo + 'd,
    B: Ingester<'d, D, Delete<'d, D>>
        + Ingester<'d, D, Update<'d, D>>
        + Presenter<'d, D, Delete<'d, D>>
        + Presenter<'d, D, Update<'d, D>>,
{
    pub const fn new(db: &'d D, boundry: B) -> Self {
        Self { db, boundry }
    }
    pub fn user_controller(&self) -> UserController<'d, '_, D, B> {
        UserController::new(self.db, &self.boundry)
    }

    pub fn handle_endpoint<U>(
        &self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        U: Usecase<'d, D>,
        B: Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        self.user_controller().handle_usecase::<U>(input)
    }
}

use crate::db::json_file::JsonFile;

use super::controller::UserController;

pub struct JsonFileApi<'d> {
    db: &'d JsonFile,
    api: API<'d, JsonFile, UserStringBoundry>,
}
impl<'d> JsonFileApi<'d> {
    pub fn new(db: &'d JsonFile) -> Self {
        Self {
            db,
            api: API::new(db, UserStringBoundry),
        }
    }

    pub fn delete_user(&self, id: String) -> String {
        self.api.handle_endpoint::<Delete<JsonFile>>(id)
    }
}
