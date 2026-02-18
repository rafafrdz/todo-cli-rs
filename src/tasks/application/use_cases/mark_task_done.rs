use crate::tasks::application::errors::{ApplicationError, ApplicationResult};
use crate::tasks::domain::errors::DomainError;
use crate::tasks::domain::task::Task;
use crate::tasks::ports::outputs::task_repository::TaskRepository;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MarkTaskDoneCommand {
    task_id: Uuid,
}
impl MarkTaskDoneCommand {
    pub fn new(task_id: Uuid) -> Self {
        Self { task_id }
    }
}
pub trait MarkTaskDoneUseCase {
    fn execute(&mut self, cmd: MarkTaskDoneCommand) -> ApplicationResult<Task>;
}
pub struct MarkTaskDoneService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> MarkTaskDoneService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TaskRepository> MarkTaskDoneUseCase for MarkTaskDoneService<R> {
    fn execute(&mut self, cmd: MarkTaskDoneCommand) -> ApplicationResult<Task> {
        let task_id: Uuid = cmd.task_id;
        let task: Option<Task> = self.repo.find_by_id(task_id)?;
        let task: Task = match task {
            None => Err(ApplicationError::Domain(DomainError::TaskNotFound {
                id: task_id,
            })),
            Some(task) => Ok(task.mark_done()?),
        }?;
        self.repo.save(task.clone())?;
        Ok(task)
    }
}
