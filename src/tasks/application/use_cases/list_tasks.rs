use crate::tasks::application::errors::ApplicationResult;
use crate::tasks::domain::task::{Task, TaskStatus};
use crate::tasks::ports::outputs::task_repository::{TaskQuery, TaskRepository};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ListTasksCommand {
    filter_task: FilterTask,
}
impl ListTasksCommand {
    pub fn new(filter_task: FilterTask) -> Self {
        Self { filter_task }
    }
}
pub trait ListTasksUseCase {
    fn execute(&self, cmd: ListTasksCommand) -> ApplicationResult<Vec<Task>>;
}
pub struct ListTasksService<R: TaskRepository> {
    repo: R,
}

impl<R: TaskRepository> ListTasksService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl<R: TaskRepository> ListTasksUseCase for ListTasksService<R> {
    fn execute(&self, cmd: ListTasksCommand) -> ApplicationResult<Vec<Task>> {
        let task_query: TaskQuery = filter_task_to_query(cmd.filter_task);
        Ok(self.repo.list(task_query)?)
    }
}

pub fn filter_task_to_query(filter_task: FilterTask) -> TaskQuery {
    match filter_task {
        FilterTask::All => TaskQuery::All,
        FilterTask::Done => TaskQuery::ByStatus(TaskStatus::Done),
        FilterTask::Todo => TaskQuery::ByStatus(TaskStatus::Todo),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterTask {
    All,
    Done,
    Todo,
}
