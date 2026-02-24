use crate::tasks::adapters::cli::cli_command::OutputFormat;
use crate::tasks::adapters::cli::errors::CliResult;
use crate::tasks::domain::task::{Task, TaskStatus};
use serde::Serialize;

pub fn print_task(task: &Task, output: OutputFormat) -> CliResult<()> {
    match output {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(task)?);
        }
        OutputFormat::Table => {
            print_tasks_table(std::slice::from_ref(task));
        }
    }
    Ok(())
}

pub fn print_tasks(tasks: &[Task], output: OutputFormat) -> CliResult<()> {
    match output {
        OutputFormat::Json => {
            println!("{}", serde_json::to_string(tasks)?);
        }
        OutputFormat::Table => {
            print_tasks_table(tasks);
        }
    }
    Ok(())
}

pub fn print_delete(id: String, deleted: bool, output: OutputFormat) -> CliResult<()> {
    let message = if deleted {
        format!("deleted {id}")
    } else {
        format!("task {id} not found")
    };

    match output {
        OutputFormat::Json => {
            let payload = DeleteOutput {
                id,
                deleted,
                message,
            };
            println!("{}", serde_json::to_string(&payload)?);
        }
        OutputFormat::Table => {
            let result = if deleted { "DELETED" } else { "NOT_FOUND" };
            println!("| RESULT    | MESSAGE            |");
            println!("|-----------|--------------------|");
            println!("| {result:<9} | {message} |");
        }
    }

    Ok(())
}

fn print_tasks_table(tasks: &[Task]) {
    let id_header = "ID";
    let status_header = "STATUS";
    let title_header = "TITLE";

    let id_width = tasks
        .iter()
        .map(|task| task.task_id().to_string().len())
        .max()
        .unwrap_or(0)
        .max(id_header.len());
    let status_width = tasks
        .iter()
        .map(|task| status_label(task).len())
        .max()
        .unwrap_or(0)
        .max(status_header.len());
    let title_width = tasks
        .iter()
        .map(|task| task.title().len())
        .max()
        .unwrap_or(0)
        .max(title_header.len());

    println!(
        "| {:<id_width$} | {:<status_width$} | {:<title_width$} |",
        id_header, status_header, title_header
    );
    println!(
        "|-{:-<id_width$}-|-{:-<status_width$}-|-{:-<title_width$}-|",
        "", "", ""
    );

    for task in tasks {
        let id = task.task_id().to_string();
        println!(
            "| {:<id_width$} | {:<status_width$} | {:<title_width$} |",
            id,
            status_label(task),
            task.title()
        );
    }
}

fn status_label(task: &Task) -> &'static str {
    match task.status() {
        TaskStatus::Todo => "TODO",
        TaskStatus::Done => "DONE",
    }
}

#[derive(Debug, Serialize)]
struct DeleteOutput {
    id: String,
    deleted: bool,
    message: String,
}
