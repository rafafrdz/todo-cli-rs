use clap::Parser;
use todo_cli::tasks::adapters::cli::cli_command::{Cli, TodoCommand};
use todo_cli::tasks::adapters::cli::errors::CliResult;
use todo_cli::tasks::adapters::persistence::json_file_task_repository::{
    JsonFileTaskRepository, TasksFile,
};
use todo_cli::tasks::application::errors::ApplicationError;
use todo_cli::tasks::application::use_cases::add_task::{
    AddTaskCommand, AddTaskService, AddTaskUseCase,
};
use todo_cli::tasks::application::use_cases::delete_task::{
    DeleteTaskCommand, DeleteTaskService, DeleteTaskUseCase,
};
use todo_cli::tasks::application::use_cases::list_tasks::{
    ListTasksCommand, ListTasksService, ListTasksUseCase,
};
use todo_cli::tasks::application::use_cases::mark_task_done::{
    MarkTaskDoneCommand, MarkTaskDoneService, MarkTaskDoneUseCase,
};
use todo_cli::tasks::application::use_cases::mark_task_todo::{
    MarkTaskTodoCommand, MarkTaskTodoService, MarkTaskTodoUseCase,
};
use todo_cli::tasks::domain::task::Task;

/// CLI Contract v0.1
/// - add <title>
///   - Input: title: String (required)
///   - Rules: title must not be empty/blank
///   - Success output: created task summary (id, title, status)
///   - Error output: validation error when title is empty
/// - list [--status <all|todo|done>]
///   - Input: optional status flag
///   - Default: all
///   - Success output: one line per task with status + id + title
///   - Error output: invalid status value (argument parsing)
/// - done <id>
///   - Input: id: Uuid (required)
///   - Rules: task must exist
///   - Success output: updated task summary (id, status=Done)
///   - Error output: invalid UUID format or task not found
/// - todo <id>
///   - Input: id: Uuid (required)
///   - Rules: task must exist
///   - Success output: updated task summary (id, status=Todo)
///   - Error output: invalid UUID format or task not found
/// - delete <id>
///   - Input: id: Uuid (required)
///   - Behavior: idempotent-style response
///   - Success output:
///     - if found: deleted <id>
///     - if not found: task <id> not found
///   - Error output: invalid UUID format
pub fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> CliResult<()> {
    let cli = Cli::parse();
    let repository = JsonFileTaskRepository::new().map_err(ApplicationError::Repository)?;
    match cli.command {
        TodoCommand::Add { title } => {
            let mut add_service = AddTaskService::new(repository);
            let task: Task = add_service.execute(AddTaskCommand::new(title))?;
            println!("{}", serde_json::to_string(&task)?);
            Ok(())
        }
        TodoCommand::List { status } => {
            let list_service = ListTasksService::new(repository);
            let tasks: Vec<Task> = list_service.execute(ListTasksCommand::from(status))?;
            let task_file: TasksFile = TasksFile::from(tasks);
            println!("{}", serde_json::to_string(&task_file)?);
            Ok(())
        }
        TodoCommand::Done { id } => {
            let mut mark_task_done_service = MarkTaskDoneService::new(repository);
            let _ = mark_task_done_service.execute(MarkTaskDoneCommand::new(id))?;
            println!("done command received (id={id})");
            Ok(())
        }
        TodoCommand::Todo { id } => {
            let mut mark_task_todo_service = MarkTaskTodoService::new(repository);
            let _ = mark_task_todo_service.execute(MarkTaskTodoCommand::new(id))?;
            println!("todo command received (id={id})");
            Ok(())
        }
        TodoCommand::Delete { id } => {
            let mut delete_service = DeleteTaskService::new(repository);
            let deleted = delete_service.execute(DeleteTaskCommand::new(id))?;
            if deleted {
                println!("delete command received (id={id})");
                Ok(())
            } else {
                Ok(())
            }
        }
    }
}
