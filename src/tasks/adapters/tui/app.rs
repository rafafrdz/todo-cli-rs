use crate::tasks::adapters::tui::errors::TuiResult;
use crate::tasks::application::use_cases::add_task::{
    AddTaskCommand, AddTaskService, AddTaskUseCase,
};
use crate::tasks::application::use_cases::delete_task::{
    DeleteTaskCommand, DeleteTaskService, DeleteTaskUseCase,
};
use crate::tasks::application::use_cases::edit_task::{
    EditTaskCommand, EditTaskService, EditTaskUseCase,
};
use crate::tasks::application::use_cases::list_tasks::{
    FilterTask, ListTasksCommand, ListTasksService, ListTasksUseCase,
};
use crate::tasks::application::use_cases::mark_task_done::{
    MarkTaskDoneCommand, MarkTaskDoneService, MarkTaskDoneUseCase,
};
use crate::tasks::application::use_cases::mark_task_todo::{
    MarkTaskTodoCommand, MarkTaskTodoService, MarkTaskTodoUseCase,
};
use crate::tasks::domain::task::{Task, TaskStatus};
use crate::tasks::ports::outputs::task_repository::TaskRepository;
use std::cmp::min;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum InputMode {
    Normal,
    Adding,
    Editing,
    ConfirmDelete,
}

pub struct App<R: TaskRepository> {
    pub tasks: Vec<Task>,
    pub selected: usize,
    pub input_mode: InputMode,
    pub input_buffer: String,
    pub status_message: Option<String>,
    pub filter: FilterTask,
    pub should_quit: bool,
    repo: R,
}

impl<R: TaskRepository + Clone> App<R> {
    pub fn new(repository: R) -> TuiResult<Self> {
        let mut app = Self {
            tasks: Vec::default(),
            selected: usize::default(),
            input_mode: InputMode::Normal,
            input_buffer: String::default(),
            status_message: None,
            filter: FilterTask::All,
            should_quit: false,
            repo: repository,
        };
        app.refresh_tasks()?;
        Ok(app)
    }

    pub fn refresh_tasks(&mut self) -> TuiResult<()> {
        let list_service: ListTasksService<R> = ListTasksService::new(self.repo.clone());
        let tasks: Vec<Task> = list_service.execute(ListTasksCommand::new(self.filter))?;
        let len = tasks.len();
        self.tasks = tasks;
        self.selected = min(self.selected, len.saturating_sub(1));
        Ok(())
    }

    pub fn add_task(&mut self) -> TuiResult<()> {
        self.status_message = None;
        let mut add_task_service: AddTaskService<R> = AddTaskService::new(self.repo.clone());
        match add_task_service.execute(AddTaskCommand::new(self.input_buffer.clone())) {
            Ok(task) => {
                self.status_message = Some(format!("Task added: {}", task.title()));
            }
            Err(e) => self.status_message = Some(format!("Error: {e}")),
        }
        self.input_buffer = String::default();
        self.input_mode = InputMode::Normal;
        self.refresh_tasks()
    }

    pub fn delete_task(&mut self) -> TuiResult<()> {
        self.status_message = None;
        let title = self.tasks[self.selected].title().to_string();
        let mut delete_task_service: DeleteTaskService<R> =
            DeleteTaskService::new(self.repo.clone());
        let task_id = self.tasks[self.selected].task_id();
        match delete_task_service.execute(DeleteTaskCommand::new(task_id)) {
            Ok(true) => {
                self.status_message = Some(format!("Deleted: {title}"));
            }
            Ok(false) => {
                self.status_message = Some("Task not found".to_string());
            }
            Err(e) => self.status_message = Some(format!("Error: {e}")),
        }
        self.input_mode = InputMode::Normal;
        self.refresh_tasks()
    }

    fn mark_done(&mut self) -> TuiResult<()> {
        self.status_message = None;
        let mut mark_done_service: MarkTaskDoneService<R> =
            MarkTaskDoneService::new(self.repo.clone());
        let task_id = self.tasks[self.selected].task_id();
        match mark_done_service.execute(MarkTaskDoneCommand::new(task_id)) {
            Ok(task) => {
                self.status_message = Some(format!("Done: {}", task.title()));
            }
            Err(e) => self.status_message = Some(format!("Error: {e}")),
        }
        self.refresh_tasks()
    }

    fn mark_todo(&mut self) -> TuiResult<()> {
        self.status_message = None;
        let mut mark_todo_service: MarkTaskTodoService<R> =
            MarkTaskTodoService::new(self.repo.clone());
        let task_id = self.tasks[self.selected].task_id();
        match mark_todo_service.execute(MarkTaskTodoCommand::new(task_id)) {
            Ok(task) => {
                self.status_message = Some(format!("Todo: {}", task.title()));
            }
            Err(e) => self.status_message = Some(format!("Error: {e}")),
        }
        self.refresh_tasks()
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    pub fn start_editing(&mut self) {
        let title = self.tasks[self.selected].title().to_string();
        self.input_buffer = title;
        self.input_mode = InputMode::Editing;
    }

    pub fn edit_task(&mut self) -> TuiResult<()> {
        self.status_message = None;
        let task_id = self.tasks[self.selected].task_id();
        let mut edit_service: EditTaskService<R> = EditTaskService::new(self.repo.clone());
        match edit_service.execute(EditTaskCommand::new(task_id, self.input_buffer.clone())) {
            Ok(task) => {
                self.status_message = Some(format!("Edited: {}", task.title()));
            }
            Err(e) => self.status_message = Some(format!("Error: {e}")),
        }
        self.input_buffer = String::default();
        self.input_mode = InputMode::Normal;
        self.refresh_tasks()
    }

    pub fn cycle_todo_done(&mut self) -> TuiResult<()> {
        let task_status = self.tasks[self.selected].status();
        match task_status {
            TaskStatus::Todo => self.mark_done(),
            TaskStatus::Done => self.mark_todo(),
        }
    }

    pub fn cycle_filter(&mut self) -> TuiResult<()> {
        match self.filter {
            FilterTask::All => self.filter = FilterTask::Done,
            FilterTask::Done => self.filter = FilterTask::Todo,
            FilterTask::Todo => self.filter = FilterTask::All,
        }
        self.refresh_tasks()
    }

    pub fn select_next(&mut self) -> TuiResult<()> {
        if self.selected != self.tasks.len().saturating_sub(1) {
            self.selected += 1;
        }
        Ok(())
    }

    pub fn select_previous(&mut self) -> TuiResult<()> {
        if self.selected != 0 {
            self.selected -= 1;
        }
        Ok(())
    }
}
