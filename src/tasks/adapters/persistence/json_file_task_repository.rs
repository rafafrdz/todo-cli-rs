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
