use crate::application::gateway::repository as repo;
use crate::application::usecase::user::{self, delete::Delete, update::Update};
pub trait Usecase<'d, D>
where
    D: repo::user::Repo,
{
    type Request;
    type Response;
    type Error;
    fn exec_u(&self, req: Self::Request) -> Result<Self::Response, Self::Error>;
    fn new_u(db: &'d D) -> Self;
}

pub mod error {
    pub mod user {
        pub mod delete {

            use crate::application::usecase::user::delete as uc;

            use thiserror::Error;

            #[derive(Debug, Error)]
            pub enum Error {
                #[error("Id parse error")]
                ParseIdError,
                #[error("{}", uc::Error::NotFound)]
                NotFound,
                #[error("{}", uc::Error::Repo)]
                Repo,
            }

            impl From<uc::Error> for Error {
                fn from(e: uc::Error) -> Self {
                    match e {
                        uc::Error::Repo => Error::Repo,
                        uc::Error::NotFound => Error::NotFound,
                    }
                }
            }
        }
        pub mod update {
            use crate::{
                application::usecase::user::{update as uc, validate::UserInvalidity},
                domain::entity::user::Id,
            };
            use thiserror::Error;

            #[derive(Debug, Error)]
            pub enum Error {
                #[error("Id parse error")]
                ParseIdError,
                #[error("User {0:?} not found")]
                NotFound(Id),
                #[error("{}", uc::Error::Repo)]
                Repo,
                #[error(transparent)]
                Invalidity(#[from] UserInvalidity),
            }

            impl From<uc::Error> for Error {
                fn from(from: uc::Error) -> Self {
                    match from {
                        uc::Error::NotFound(id) => Self::NotFound(id.into()),
                        uc::Error::Invalidity(i) => Self::Invalidity(i),
                        uc::Error::Repo => Self::Repo,
                    }
                }
            }
        }
    }
}
use error::user as user_error;
pub trait Ingester<'a, A, U: Usecase<'a, A>>
where
    A: repo::user::Repo,
{
    type InputModel;
    fn ingest(&self, input: Self::InputModel) -> Result<U::Request, U::Error>;
}
pub trait Presenter<'a, A, U: Usecase<'a, A>>
where
    A: repo::user::Repo,
{
    type ViewModel;
    fn present(&self, data: Result<U::Response, U::Error>) -> Self::ViewModel;
}

pub trait Boundry<'a, A, U: Usecase<'a, A>>: Ingester<'a, A, U> + Presenter<'a, A, U>
where
    A: repo::user::Repo,
{
}

impl<'u, U> Usecase<'u, U> for Delete<'u, U>
where
    U: repo::user::Repo,
{
    type Request = user::delete::Request;
    type Response = user::delete::Response;
    type Error = user_error::delete::Error;
    fn exec_u(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.exec(req).map_err(Self::Error::from)
    }
    fn new_u(db: &'u U) -> Self {
        Self::new(db)
    }
}

impl<'u, U> Usecase<'u, U> for Update<'u, U>
where
    U: repo::user::Repo,
{
    type Request = user::update::Request;
    type Response = user::update::Response;
    type Error = user_error::update::Error;
    fn exec_u(&self, req: Self::Request) -> Result<Self::Response, Self::Error> {
        self.exec(req).map_err(Self::Error::from)
    }
    fn new_u(db: &'u U) -> Self {
        Self::new(db)
    }
}

pub struct Controller<'d, 'b, D, B> {
    db: &'d D,
    boundry: &'b B,
}

impl<'d, 'b, D, B> Controller<'d, 'b, D, B>
where
    D: repo::user::Repo,
    B: Boundry<'d, D, Delete<'d, D>> + Boundry<'d, D, Update<'d, D>>,
{
    pub const fn new(db: &'d D, boundry: &'b B) -> Self {
        Self { db, boundry }
    }

    pub fn handle_usecase<U>(
        &self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        U: Usecase<'d, D>,
        B: Boundry<'d, D, U> + Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        let req = <B as Ingester<'d, D, U>>::ingest(self.boundry, input)
            .and_then(|req| U::new_u(self.db).exec_u(req));
        <B as Presenter<'d, D, U>>::present(self.boundry, req)
    }
}

pub struct UserStringBoundry;
impl<'r, R> Ingester<'r, R, Delete<'r, R>> for UserStringBoundry
where
    R: repo::user::Repo,
{
    // <B as Ingester<'d, D, U>>::InputModel
    type InputModel = String;
    fn ingest(
        &self,
        input: String,
    ) -> Result<<Delete<'r, R> as Usecase<'r, R>>::Request, <Delete<'r, R> as Usecase<'r, R>>::Error>
    {
        uuid::Uuid::parse_str(&input)
            .map_err(|_| user_error::delete::Error::ParseIdError)
            .map(|id| user::delete::Request { id: id.into() })
    }
}
impl<'r, R> Presenter<'r, R, Delete<'r, R>> for UserStringBoundry
where
    R: repo::user::Repo,
{
    type ViewModel = String;
    fn present(
        &self,
        data: Result<
            <Delete<'r, R> as Usecase<'r, R>>::Response,
            <Delete<'r, R> as Usecase<'r, R>>::Error,
        >,
    ) -> Self::ViewModel {
        match data {
            Ok(_) => "Deleted User".to_string(),
            Err(_) => "Unable to delete user".to_string(),
        }
    }
}

impl<'r, R> Boundry<'r, R, Delete<'r, R>> for UserStringBoundry where R: repo::user::Repo {}

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
        &self,
        input: Self::InputModel,
    ) -> Result<<Update<'r, R> as Usecase<'r, R>>::Request, <Update<'r, R> as Usecase<'r, R>>::Error>
    {
        uuid::Uuid::parse_str(&input.id)
            .map_err(|_| user_error::update::Error::ParseIdError)
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
        &self,
        data: Result<
            <Update<'r, R> as Usecase<'r, R>>::Response,
            <Update<'r, R> as Usecase<'r, R>>::Error,
        >,
    ) -> Self::ViewModel {
        match data {
            Ok(_) => "Updated User".to_string(),
            Err(_) => "Unable to update user".to_string(),
        }
    }
}
impl<'r, R> Boundry<'r, R, Update<'r, R>> for UserStringBoundry where R: repo::user::Repo {}

pub struct API<'d, D, B> {
    db: &'d D,
    boundry: B,
}

impl<'d, D, B> API<'d, D, B>
where
    D: repo::user::Repo + 'd,
    B: Boundry<'d, D, Delete<'d, D>> + Boundry<'d, D, Update<'d, D>>,
{
    pub const fn new(db: &'d D, boundry: B) -> Self {
        Self { db, boundry }
    }
    pub fn controller(&self) -> Controller<'d, '_, D, B> {
        Controller::new(self.db, &self.boundry)
    }

    pub fn handle_endpoint<U>(
        &self,
        input: <B as Ingester<'d, D, U>>::InputModel,
    ) -> <B as Presenter<'d, D, U>>::ViewModel
    where
        U: Usecase<'d, D>,
        B: Boundry<'d, D, U> + Ingester<'d, D, U> + Presenter<'d, D, U>,
    {
        self.controller().handle_usecase::<U>(input)
    }
}

use crate::db::json_file::JsonFile;

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
