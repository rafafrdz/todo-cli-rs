use crate::tasks::application::errors::{ApplicationError, ApplicationResult};
use crate::tasks::domain::errors::DomainError;
use crate::tasks::domain::task::Task;
use crate::tasks::ports::outputs::task_repository::TaskRepository;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkTaskTodoCommand {
    task_id: Uuid,
}
impl MarkTaskTodoCommand {
    pub fn new(task_id: Uuid) -> Self {
        Self { task_id }
    }
}
pub trait MarkTaskTodoUseCase {
    fn execute(&mut self, cmd: MarkTaskTodoCommand) -> ApplicationResult<Task>;
}
pub struct MarkTaskTodoService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> MarkTaskTodoService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TaskRepository> MarkTaskTodoUseCase for MarkTaskTodoService<R> {
    fn execute(&mut self, cmd: MarkTaskTodoCommand) -> ApplicationResult<Task> {
        let task_id: Uuid = cmd.task_id;
        let task: Option<Task> = self.repo.find_by_id(task_id)?;
        let task: Task = match task {
            None => Err(ApplicationError::Domain(DomainError::TaskNotFound {
                id: task_id,
            })),
            Some(task) => Ok(task.mark_todo()?),
        }?;
        self.repo.save(task.clone())?;
        Ok(task)
    }
}
