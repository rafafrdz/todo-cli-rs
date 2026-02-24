use crate::tasks::domain::task::Task;
use crate::tasks::ports::outputs::errors::{RepoError, RepoResult};
use crate::tasks::ports::outputs::task_repository::{TaskQuery, TaskRepository};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug)]
pub struct JsonFileTaskRepository {
    file_path: PathBuf,
}

impl JsonFileTaskRepository {
    pub fn new() -> RepoResult<Self> {
        let project_dirs = ProjectDirs::from("com", "org", "todo-cli").ok_or_else(|| {
            RepoError::InternalError {
                error: "could not resolve project directories".to_string(),
            }
        })?;
        let data_dir = project_dirs.config_dir().join("data");
        fs::create_dir_all(&data_dir).map_err(|e| RepoError::InternalError {
            error: format!(
                "could not create data directory '{}': {e}",
                data_dir.display()
            ),
        })?;
        let file_path = data_dir.join("tasks.json");
        Ok(Self { file_path })
    }
    pub fn using(file_path: PathBuf) -> Self {
        Self { file_path }
    }
    pub fn file_path(&self) -> &Path {
        &self.file_path
    }

    fn read_task_file(&self) -> RepoResult<TasksFile> {
        if !self.file_path.exists() {
            Ok(TasksFile::default())
        } else {
            let file =
                fs::read_to_string(&self.file_path).map_err(|e| RepoError::InternalError {
                    error: format!("Reading data from file. E: {e:?}"),
                })?;
            serde_json::from_str(file.as_str()).map_err(|e| RepoError::InternalError {
                error: format!("Parsing data from file to tasks. E: {e:?}"),
            })
        }
    }

    fn write_tasks_file(&self, tasks_file: &TasksFile) -> RepoResult<()> {
        if let Some(parent) = self.file_path.parent() {
            fs::create_dir_all(parent).map_err(|e| RepoError::InternalError {
                error: format!(
                    "could not create parent directory '{}': {e}",
                    parent.display()
                ),
            })?;
        }

        let payload = serde_json::to_string(tasks_file).map_err(|e| RepoError::InternalError {
            error: format!("Serializing data. E: {e:?}"),
        })?;

        fs::write(&self.file_path, payload).map_err(|e| RepoError::InternalError {
            error: format!("Writing data. E: {e:?}"),
        })
    }
}

impl TaskRepository for JsonFileTaskRepository {
    fn save(&mut self, task: Task) -> RepoResult<()> {
        let mut tasks_file = self.read_task_file()?;

        if let Some(index) = tasks_file
            .tasks
            .iter()
            .position(|stored| stored.task_id() == task.task_id())
        {
            tasks_file.tasks[index] = task;
        } else {
            tasks_file.tasks.push(task);
        }

        self.write_tasks_file(&tasks_file)
    }

    fn list(&self, query: TaskQuery) -> RepoResult<Vec<Task>> {
        let TasksFile { tasks } = self.read_task_file()?;
        match query {
            TaskQuery::All => Ok(tasks),
            TaskQuery::ByStatus(status) => Ok(tasks
                .iter()
                .cloned()
                .filter(|t| t.status() == status)
                .collect()),
        }
    }

    fn find_by_id(&self, id: Uuid) -> RepoResult<Option<Task>> {
        let TasksFile { tasks } = self.read_task_file()?;
        Ok(tasks.iter().cloned().find(|t| t.task_id() == id))
    }

    fn delete(&mut self, id: Uuid) -> RepoResult<bool> {
        let mut tasks_file = self.read_task_file()?;
        let initial_len = tasks_file.tasks.len();
        tasks_file.tasks.retain(|task| task.task_id() != id);

        if tasks_file.tasks.len() == initial_len {
            return Ok(false);
        }

        self.write_tasks_file(&tasks_file)?;
        Ok(true)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct TasksFile {
    tasks: Vec<Task>,
}

impl Default for TasksFile {
    fn default() -> Self {
        TasksFile {
            tasks: Vec::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JsonFileTaskRepository;
    use crate::tasks::domain::task::{Task, TaskStatus};
    use crate::tasks::ports::outputs::task_repository::{TaskQuery, TaskRepository};
    use std::fs;
    use tempfile::tempdir;

    fn new_task(title: &str) -> Task {
        Task::new(title.to_string()).expect("task should be created")
    }

    #[test]
    fn save_creates_file_and_persists_task() {
        let temp = tempdir().expect("temp dir should be created");
        let file_path = temp.path().join("tasks.json");
        let mut repo = JsonFileTaskRepository::using(file_path.clone());

        let task = new_task("learn rust");
        let id = task.task_id();

        repo.save(task).expect("save should succeed");

        assert!(file_path.exists());
        let found = repo.find_by_id(id).expect("find should succeed");
        assert!(found.is_some());
    }

    #[test]
    fn list_all_returns_tasks_persisted_on_disk() {
        let temp = tempdir().expect("temp dir should be created");
        let file_path = temp.path().join("tasks.json");
        let mut repo = JsonFileTaskRepository::using(file_path);

        repo.save(new_task("task 1")).expect("save should succeed");
        repo.save(new_task("task 2")).expect("save should succeed");

        let all = repo.list(TaskQuery::All).expect("list should succeed");
        assert_eq!(all.len(), 2);
    }

    #[test]
    fn list_by_status_filters_tasks() {
        let temp = tempdir().expect("temp dir should be created");
        let file_path = temp.path().join("tasks.json");
        let mut repo = JsonFileTaskRepository::using(file_path);

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
    }

    #[test]
    fn delete_returns_true_for_existing_task_and_false_otherwise() {
        let temp = tempdir().expect("temp dir should be created");
        let file_path = temp.path().join("tasks.json");
        let mut repo = JsonFileTaskRepository::using(file_path);

        let task = new_task("task to delete");
        let id = task.task_id();
        repo.save(task).expect("save should succeed");

        let deleted = repo.delete(id).expect("delete should succeed");
        assert!(deleted);

        let deleted_again = repo.delete(id).expect("delete should succeed");
        assert!(!deleted_again);
    }

    #[test]
    fn data_persists_between_repository_instances() {
        let temp = tempdir().expect("temp dir should be created");
        let file_path = temp.path().join("tasks.json");

        let mut writer = JsonFileTaskRepository::using(file_path.clone());
        let task = new_task("persist me");
        let id = task.task_id();
        writer.save(task).expect("save should succeed");

        let reader = JsonFileTaskRepository::using(file_path);
        let found = reader.find_by_id(id).expect("find should succeed");
        assert!(found.is_some());
    }

    #[test]
    fn invalid_json_returns_error() {
        let temp = tempdir().expect("temp dir should be created");
        let file_path = temp.path().join("tasks.json");
        fs::write(&file_path, "{invalid json").expect("invalid test payload should be written");

        let repo = JsonFileTaskRepository::using(file_path);
        let result = repo.list(TaskQuery::All);

        assert!(result.is_err());
    }
}
