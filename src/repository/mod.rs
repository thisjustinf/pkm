use thiserror::Error;
pub mod note_repository;

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("There was an unknown DB error")]
    InsertError(#[from] diesel::result::Error),
    #[error("Resource not found in the DB")]
    ResourceNotFound,
    #[error("Database Error: {0}")]
    Database(#[from] diesel::result::Error)
}

pub trait Repository<T, U> {
    fn get_all(&self) -> Result<T, RepositoryError>;
    fn get_by_id(&self, id: U) -> Result<T, RepositoryError>;
    fn create<I>(&self, insertable: &I) -> Result<T, RepositoryError>;
    fn update(&self, id: U) -> Result<T, RepositoryError>;
    fn delete(&self, id: U) -> Result<T, RepositoryError>;
}
