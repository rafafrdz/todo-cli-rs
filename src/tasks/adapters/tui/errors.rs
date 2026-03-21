use crate::tasks::application::errors::ApplicationError;
use std::io;
use thiserror::Error;
pub type TuiResult<T> = Result<T, TuiError>;
#[derive(Debug, Error)]
pub enum TuiError {
    #[error(transparent)]
    Application(#[from] ApplicationError),
    #[error(transparent)]
    Io(#[from] io::Error),
}
