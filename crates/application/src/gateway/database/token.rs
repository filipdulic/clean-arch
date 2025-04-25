use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error, Serialize, PartialEq, Clone)]
pub enum GenError {
    #[error("Token repository connection problem")]
    Connection,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum VerifyError {
    #[error("Token not found")]
    NotFound,
    #[error("Token repository connection problem")]
    Connection,
    #[error("Token mismatch")]
    Mismatch,
    #[error("Token expired")]
    TokenExpired,
}

#[derive(Debug, Error, Serialize, PartialEq)]
pub enum ExtendError {
    #[error("Token repository connection problem")]
    Connection,
    #[error("Token not found")]
    NotFound,
}

#[derive(Debug, Clone)]
pub struct Record {
    pub token: String,
}
#[cfg_attr(test, mockall::automock(type Transaction = ();))]
#[async_trait::async_trait]
pub trait Repo: Send + Sync {
    type Transaction;
    async fn gen(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        email: &str,
    ) -> Result<Record, GenError>;
    async fn verify(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        email: &str,
        token: &str,
    ) -> Result<(), VerifyError>;
    async fn extend(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        email: &str,
    ) -> Result<(), ExtendError>;
}

#[cfg(test)]
#[async_trait::async_trait]
impl Repo for &MockRepo {
    type Transaction = ();
    async fn gen(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        email: &str,
    ) -> Result<Record, GenError> {
        (*self).gen(transaction, email).await
    }
    async fn verify(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        email: &str,
        token: &str,
    ) -> Result<(), VerifyError> {
        (*self).verify(transaction, email, token).await
    }
    async fn extend(
        &self,
        transaction: Option<Arc<futures::lock::Mutex<Self::Transaction>>>,
        email: &str,
    ) -> Result<(), ExtendError> {
        (*self).extend(transaction, email).await
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    async fn test_mock() {
        // Create a mock instance
        let mut mock = MockRepo::new();

        // email
        const EMAIL: &str = "test@email.com";
        const RETURN_TOKEN: &str = "test_token";

        // Set up expectations
        mock.expect_gen()
            .withf(move |transaction, actual_email| transaction.is_none() && actual_email == EMAIL)
            .times(1)
            .returning(|_, _| {
                Ok(Record {
                    token: RETURN_TOKEN.to_string(),
                })
            });

        // Call the method
        let result = mock.gen(None, EMAIL).await;

        // Verify the result
        assert!(result.is_ok());
        let record = result.unwrap();
        assert_eq!(record.token, RETURN_TOKEN);
    }
}
