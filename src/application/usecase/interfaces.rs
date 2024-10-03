// use std::future::Future;

// use crate::domain::ApiError;

// pub trait AbstractUseCase<T> {
//     // async fn execute(&self) -> Result<T, ApiError>;
//     fn execute(&self) -> impl Future<Output = Result<T, ApiError>> + Send;
// }