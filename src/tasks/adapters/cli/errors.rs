use crate::tasks::application::errors::ApplicationError;
use thiserror::Error;

pub type CliResult<T> = Result<T, CliError>;
#[derive(Debug, Error)]
pub enum CliError {
    #[error(transparent)]
    Application(#[from] ApplicationError),
    #[error(transparent)]
    Serializer(#[from] serde_json::Error),
}
