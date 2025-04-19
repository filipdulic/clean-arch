use std::future::Future;

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize, PartialEq)]
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

#[derive(Debug)]
pub struct Record {
    pub token: String,
}

pub trait Repo: Send + Sync {
    type Transaction;
    fn gen(
        &self,
        transaction: Option<&mut Self::Transaction>,
        email: &str,
    ) -> impl Future<Output = Result<Record, GenError>>;
    fn verify(
        &self,
        transaction: Option<&mut Self::Transaction>,
        email: &str,
        token: &str,
    ) -> impl Future<Output = Result<(), VerifyError>>;
    fn extend(
        &self,
        transaction: Option<&mut Self::Transaction>,
        email: &str,
    ) -> impl Future<Output = Result<(), ExtendError>>;
}

#[cfg(test)]
pub mod mock {
    use super::*;
    use mockall::mock;
    mock! {
        pub TokenRepo {}
        impl Repo for  TokenRepo{
            type Transaction = ();
            fn gen<'a>(
                &self,
                transaction: Option<&'a mut <MockTokenRepo as Repo>::Transaction>,
                email: &str,
            ) -> impl Future<Output = Result<Record, GenError>>;
            fn verify<'a>(
                &self,
                transaction: Option<&'a mut <MockTokenRepo as Repo>::Transaction>,
                email: &str,
                token: &str,
            ) -> impl Future<Output = Result<(), VerifyError>>;
            fn extend<'a>(
                &self,
                transaction: Option<&'a mut <MockTokenRepo as Repo>::Transaction>,
                email: &str,
            ) -> impl Future<Output = Result<(), ExtendError>>;
        }
    }

    impl Repo for &MockTokenRepo {
        type Transaction = ();
        fn gen<'a>(
            &self,
            transaction: Option<&'a mut <MockTokenRepo as Repo>::Transaction>,
            email: &str,
        ) -> impl Future<Output = Result<Record, GenError>> {
            (*self).gen(transaction, email)
        }
        fn verify<'a>(
            &self,
            transaction: Option<&'a mut <MockTokenRepo as Repo>::Transaction>,
            email: &str,
            token: &str,
        ) -> impl Future<Output = Result<(), VerifyError>> {
            (*self).verify(transaction, email, token)
        }
        fn extend<'a>(
            &self,
            transaction: Option<&'a mut <MockTokenRepo as Repo>::Transaction>,
            email: &str,
        ) -> impl Future<Output = Result<(), ExtendError>> {
            (*self).extend(transaction, email)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mock() {
        // Create a mock instance
        let mut mock = mock::MockTokenRepo::new();

        // email
        const EMAIL: &str = "test@email.com";
        const RETURN_TOKEN: &str = "test_token";

        // Set up expectations
        mock.expect_gen()
            .withf(move |transaction, actual_email| transaction.is_none() && actual_email == EMAIL)
            .times(1)
            .returning(|_, _| {
                Box::pin(async {
                    Ok(Record {
                        token: RETURN_TOKEN.to_string(),
                    })
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
