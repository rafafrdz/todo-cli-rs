use crate::tasks::domain::errors::DomainError;
use crate::tasks::ports::outputs::errors::RepoError;
use thiserror::Error;

pub type ApplicationResult<T> = Result<T, ApplicationError>;
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error(transparent)]
    Domain(#[from] DomainError),
    #[error(transparent)]
    Repository(#[from] RepoError),
}
