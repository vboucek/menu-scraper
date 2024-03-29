use std::sync::Arc;

use async_trait::async_trait;

use crate::db::common::error::{DbResultMultiple, DbResultSingle};

#[async_trait]
pub trait DbCreate<Create, Data> {
    /// Generic call which creates a record in the database
    ///
    /// # Arguments
    ///
    /// - `self`: reference to the repository to access the pool handler
    /// - `data`: the structure which passes all the data that is necessary for creation of the
    ///         record in the database
    ///
    /// # Returns
    ///
    /// - `Ok(Data)` on success (the provided structure which represents
    ///                          data coming from the database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn create(&self, data: &Create) -> DbResultSingle<Data>;
}

#[async_trait]
pub trait DbReadOne<ReadOne, Data> {
    /// Generic call which reads a single record from the database
    ///
    /// # Arguments
    ///
    /// - `self`: reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the read operation
    ///
    /// # Returns
    ///
    /// - `Ok(Data)` on success (the provided structure which represents read data coming
    ///                          from the database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn read_one(&self, params: &ReadOne) -> DbResultSingle<Data>;
}

#[async_trait]
pub trait DbReadMany<ReadMany, Data> {
    /// Generic call which reads multiple records from the database
    ///
    /// # Arguments
    ///
    /// - `self`: reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the read operation
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Data>)` on success (a vector of structures which represent read data from the
    ///                               database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn read_many(&self, params: &ReadMany) -> DbResultMultiple<Data>;
}

#[async_trait]
pub trait DbUpdate<Update, Data> {
    /// Generic call which updates record(s) present in the database
    ///
    /// # Arguments
    ///
    /// - `self`: reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the update operation
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Data>)` on success (a vector of structures which represent updated data from the
    ///                               database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn update(&self, params: &Update) -> DbResultMultiple<Data>;
}

#[async_trait]
pub trait DbDelete<Delete, Data> {
    /// Generic call which deletes record(s) present in the database
    ///
    /// # Arguments
    ///
    /// - `self`: mutable reference to the repository to access the pool handler
    /// - `params`: the structure which passes parameters for the delete operation
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<Data>)` on success (a vector of structures which represent deleted data from the
    ///                               database)
    /// - `sqlx::Error(_)` on any failure (SQL, DB constraints, connection, etc.)
    async fn delete(&self, params: &Delete) -> DbResultMultiple<Data>;
}

#[async_trait]
pub trait DbPoolHandler {
    /// Pool handler constructor
    #[must_use]
    fn new(pool: Arc<sqlx::PgPool>) -> Self;
}

/// Generic Postgres pool handler for repositories
#[derive(Clone)]
pub struct PoolHandler {
    pub(crate) pool: Arc<sqlx::PgPool>,
}

#[async_trait]
impl DbPoolHandler for PoolHandler {
    /// Database pool constructor
    #[must_use]
    fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}

/// Database repository trait - implements a constructor, optionally implements any of the traits
/// that are defined in this file.
#[async_trait]
pub trait DbRepository {
    /// Database repository constructor
    fn new(pool_handler: PoolHandler) -> Self;
}
