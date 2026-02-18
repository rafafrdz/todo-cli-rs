use crate::tasks::domain::task::TaskStatus;
use thiserror::Error;
use uuid::Uuid;

pub type DomainResult<T> = Result<T, DomainError>;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("task title cannot be empty")]
    EmptyTitle,
    #[error("task title exceeds max length ({max})")]
    TitleTooLong { max: usize },
    #[error("task with id {id} was not found")]
    TaskNotFound { id: Uuid },
    #[error("invalid status transition for task {id}: {from:?} => {to:?}")]
    InvalidStatusTransition {
        id: Uuid,
        from: TaskStatus,
        to: TaskStatus,
    },
}
