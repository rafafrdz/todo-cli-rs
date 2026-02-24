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

#[cfg(test)]
mod tests {
    use super::InMemoryTaskRepository;
    use crate::tasks::domain::task::{Task, TaskStatus};
    use crate::tasks::ports::outputs::task_repository::{TaskQuery, TaskRepository};

    fn new_task(title: &str) -> Task {
        Task::new(title.to_string()).expect("task should be created")
    }

    #[test]
    fn save_and_find_by_id_returns_task() {
        let mut repo = InMemoryTaskRepository::new();
        let task = new_task("learn rust");
        let id = task.task_id();

        repo.save(task).expect("save should succeed");

        let found = repo.find_by_id(id).expect("find should succeed");
        assert!(found.is_some());
        let found = found.expect("task should exist");
        assert_eq!(found.task_id(), id);
        assert_eq!(found.title(), "learn rust");
    }

    #[test]
    fn list_all_returns_all_tasks() {
        let mut repo = InMemoryTaskRepository::new();
        repo.save(new_task("task 1")).expect("save should succeed");
        repo.save(new_task("task 2")).expect("save should succeed");

        let all = repo.list(TaskQuery::All).expect("list should succeed");
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn list_by_status_filters_tasks() {
        let mut repo = InMemoryTaskRepository::new();
        let todo = new_task("todo task");
        let done = new_task("done task")
            .mark_done()
            .expect("status transition should succeed");

        repo.save(todo).expect("save should succeed");
        repo.save(done).expect("save should succeed");

        let done_tasks = repo
            .list(TaskQuery::ByStatus(TaskStatus::Done))
            .expect("list should succeed");
        assert_eq!(done_tasks.len(), 1);
        assert_eq!(done_tasks[0].status(), TaskStatus::Done);

        let todo_tasks = repo
            .list(TaskQuery::ByStatus(TaskStatus::Todo))
            .expect("list should succeed");
        assert_eq!(todo_tasks.len(), 1);
        assert_eq!(todo_tasks[0].status(), TaskStatus::Todo);
    }

    #[test]
    fn delete_returns_true_for_existing_task() {
        let mut repo = InMemoryTaskRepository::new();
        let task = new_task("task to delete");
        let id = task.task_id();
        repo.save(task).expect("save should succeed");

        let deleted = repo.delete(id).expect("delete should succeed");
        assert!(deleted);

        let found = repo.find_by_id(id).expect("find should succeed");
        assert!(found.is_none());
    }

    #[test]
    fn delete_returns_false_for_non_existing_task() {
        let mut repo = InMemoryTaskRepository::new();
        let id = new_task("temporary").task_id();

        let deleted = repo.delete(id).expect("delete should succeed");
        assert!(!deleted);
    }

    #[test]
    fn save_is_upsert_when_same_id_is_saved_again() {
        let mut repo = InMemoryTaskRepository::new();
        let original = new_task("pay rent");
        let id = original.task_id();
        repo.save(original.clone()).expect("save should succeed");

        let updated = original
            .mark_done()
            .expect("status transition should succeed");
        repo.save(updated).expect("save should succeed");

        let all = repo.list(TaskQuery::All).expect("list should succeed");
        assert_eq!(all.len(), 1);

        let found = repo.find_by_id(id).expect("find should succeed");
        let found = found.expect("task should exist");
        assert_eq!(found.status(), TaskStatus::Done);
    }
}
