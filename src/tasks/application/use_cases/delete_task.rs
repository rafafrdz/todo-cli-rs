use crate::tasks::application::errors::ApplicationResult;
use crate::tasks::ports::outputs::task_repository::TaskRepository;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DeleteTaskCommand {
    task_id: Uuid,
}
impl DeleteTaskCommand {
    pub fn new(task_id: Uuid) -> Self {
        Self { task_id }
    }
}
pub trait DeleteTaskUseCase {
    fn execute(&mut self, cmd: DeleteTaskCommand) -> ApplicationResult<bool>;
}
pub struct DeleteTaskService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> DeleteTaskService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TaskRepository> DeleteTaskUseCase for DeleteTaskService<R> {
    fn execute(&mut self, cmd: DeleteTaskCommand) -> ApplicationResult<bool> {
        let task_id: Uuid = cmd.task_id;
        Ok(self.repo.delete(task_id)?)
    }
}
