use crate::tasks::domain::errors::{DomainError, DomainResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Task {
    id: Uuid,
    title: String,
    status: TaskStatus,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    Done,
}

impl Task {
    pub fn new(title: String) -> DomainResult<Self> {
        if title.trim().is_empty() {
            Err(DomainError::EmptyTitle)
        } else {
            Ok(Task {
                id: Uuid::new_v4(),
                title,
                status: TaskStatus::Todo,
                created_at: Utc::now(),
                modified_at: Utc::now(),
            })
        }
    }

    pub fn mark_done(self) -> DomainResult<Self> {
        self.mark_as(TaskStatus::Done)
    }
    pub fn mark_todo(self) -> DomainResult<Self> {
        self.mark_as(TaskStatus::Todo)
    }

    fn mark_as(self, status: TaskStatus) -> DomainResult<Self> {
        if status == self.status {
            Err(DomainError::InvalidStatusTransition {
                id: self.id,
                from: self.status,
                to: status,
            })
        } else {
            Ok(Self {
                status,
                modified_at: Utc::now(),
                ..self
            })
        }
    }
    pub fn edit_title(self, title: String) -> DomainResult<Self> {
        if title.trim().is_empty() {
            Err(DomainError::EmptyTitle)
        } else {
            Ok(Self {
                title,
                modified_at: Utc::now(),
                ..self
            })
        }
    }

    pub fn task_id(&self) -> Uuid {
        self.id
    }
    pub fn title(&self) -> &str {
        self.title.as_str()
    }
    pub fn status(&self) -> TaskStatus {
        self.status
    }
    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
    pub fn modified_at(&self) -> DateTime<Utc> {
        self.modified_at
    }
}
