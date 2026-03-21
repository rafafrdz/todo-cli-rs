use crate::tasks::application::errors::{ApplicationError, ApplicationResult};
use crate::tasks::domain::errors::DomainError;
use crate::tasks::domain::task::Task;
use crate::tasks::ports::outputs::task_repository::TaskRepository;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditTaskCommand {
    task_id: Uuid,
    new_title: String,
}

impl EditTaskCommand {
    pub fn new(task_id: Uuid, new_title: String) -> Self {
        Self { task_id, new_title }
    }
}

pub trait EditTaskUseCase {
    fn execute(&mut self, cmd: EditTaskCommand) -> ApplicationResult<Task>;
}

pub struct EditTaskService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> EditTaskService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TaskRepository> EditTaskUseCase for EditTaskService<R> {
    fn execute(&mut self, cmd: EditTaskCommand) -> ApplicationResult<Task> {
        let task_id: Uuid = cmd.task_id;
        let task: Option<Task> = self.repo.find_by_id(task_id)?;
        let task: Task = match task {
            None => Err(ApplicationError::Domain(DomainError::TaskNotFound {
                id: task_id,
            })),
            Some(task) => Ok(task.edit_title(cmd.new_title)?),
        }?;
        self.repo.save(task.clone())?;
        Ok(task)
    }
}
