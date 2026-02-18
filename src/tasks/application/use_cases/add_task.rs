use crate::tasks::application::errors::ApplicationResult;
use crate::tasks::domain::task::Task;
use crate::tasks::ports::outputs::task_repository::TaskRepository;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AddTaskCommand {
    title: String,
}
impl AddTaskCommand {
    pub fn new(title: String) -> Self {
        Self { title }
    }
}
pub trait AddTaskUseCase {
    fn execute(&mut self, cmd: AddTaskCommand) -> ApplicationResult<Task>;
}
pub struct AddTaskService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> AddTaskService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TaskRepository> AddTaskUseCase for AddTaskService<R> {
    fn execute(&mut self, command: AddTaskCommand) -> ApplicationResult<Task> {
        let task: Task = Task::new(command.title)?;
        self.repo.save(task.clone())?;
        Ok(task)
    }
}
