use thiserror::Error;
pub mod app_service;
pub mod note_service;

#[derive(Error, Debug)]
pub enum ServiceError {
    #[error("There was an error that I didn't account for")]
    UnexpectedError,
    #[error("The data could not be properly validated")]
    ValidationError,
}

pub use app_service::AppService;
pub use note_service::NoteService;
