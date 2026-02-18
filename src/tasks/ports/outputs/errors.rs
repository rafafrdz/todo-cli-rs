use thiserror::Error;

pub type RepoResult<T> = Result<T, RepoError>;
#[derive(Debug, Error)]
pub enum RepoError {
    #[error("internal error: {error}")]
    InternalError { error: String }
}