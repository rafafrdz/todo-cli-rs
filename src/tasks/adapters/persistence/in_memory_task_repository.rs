use crate::tasks::domain::task::{Task, TaskStatus};
use crate::tasks::ports::outputs::errors::RepoResult;
use crate::tasks::ports::outputs::task_repository::{TaskQuery, TaskRepository};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InMemoryTaskRepository {
    cache: HashMap<Uuid, Task>,
}

impl InMemoryTaskRepository {
    pub fn new() -> Self {
        Self {
            cache: HashMap::default(),
        }
    }

    pub fn add_task(&mut self, task: Task) -> RepoResult<()> {
        self.cache.insert(task.task_id(), task);
        Ok(())
    }

    pub fn get_task_by_id(&self, task_id: Uuid) -> Option<&Task> {
        self.cache.get(&task_id)
    }

    pub fn get_task_by_status(&self, task_status: TaskStatus) -> Vec<Task> {
        self.cache
            .values()
            .filter(|task| task.status() == task_status)
            .cloned()
            .collect()
    }

    pub fn delete_task_by_id(&mut self, task_id: Uuid) -> Option<Task> {
        self.cache.remove(&task_id)
    }
}

impl Default for InMemoryTaskRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl TaskRepository for InMemoryTaskRepository {
    fn save(&mut self, task: Task) -> RepoResult<()> {
        Self::add_task(self, task)
    }

    fn list(&self, query: TaskQuery) -> RepoResult<Vec<Task>> {
        let result = match query {
            TaskQuery::All => self.cache.values().cloned().collect(),
            TaskQuery::ByStatus(task_status) => self.get_task_by_status(task_status),
        };
        Ok(result)
    }

    fn find_by_id(&self, task_id: Uuid) -> RepoResult<Option<Task>> {
        Ok(self.get_task_by_id(task_id).cloned())
    }

    fn delete(&mut self, task_id: Uuid) -> RepoResult<bool> {
        match self.delete_task_by_id(task_id) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }
}
