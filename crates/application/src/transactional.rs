use std::future::Future;

pub trait Transactional: Clone {
    type Transaction;
    type Error;
    fn begin_transaction(&self) -> impl Future<Output = Self::Transaction>;
    fn commit_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<(), Self::Error>>;
    fn rollback_transaction(
        &self,
        transaction: Self::Transaction,
    ) -> impl Future<Output = Result<(), Self::Error>>;
}
