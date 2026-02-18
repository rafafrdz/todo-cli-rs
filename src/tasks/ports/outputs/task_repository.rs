use crate::tasks::domain::task::{Task, TaskStatus};
use crate::tasks::ports::outputs::errors::RepoResult;
use uuid::Uuid;

pub trait TaskRepository {
    fn save(&mut self, task: Task) -> RepoResult<()>;
    fn list(&self, query: TaskQuery) -> RepoResult<Vec<Task>>;
    fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Task>>;
    fn delete(&mut self, id: Uuid) -> RepoResult<bool>;
}

#[derive(Debug, Clone, Copy)]
pub enum TaskQuery {
    All,
    ByStatus(TaskStatus),
}