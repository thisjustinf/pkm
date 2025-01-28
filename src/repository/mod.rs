use diesel::r2d2::PoolError;
use thiserror::Error;

pub mod note_repository;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("There was an error inserting the value into the database")]
    InsertError,
    #[error("Resource not found in the DB")]
    ResourceNotFound,
    #[error("Database Error: {0}")]
    DatabaseError(#[from] PoolError),
    #[error("An error occurred with Diesel")]
    DieselError(#[from] diesel::result::Error),
}

pub trait Repository<T, U, V> {
    fn get_all(&self, limit: i64) -> Result<Vec<T>, RepositoryError>;
    fn get_by_id(&self, id: U) -> Result<T, RepositoryError>;
    fn create(&self, insertable: &V) -> Result<T, RepositoryError>;
    fn update(&self, id: U, dto: &V) -> Result<T, RepositoryError>;
    fn delete(&self, id: U) -> Result<bool, RepositoryError>;
}
