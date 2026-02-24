use crate::tasks::application::use_cases::list_tasks::{FilterTask, ListTasksCommand};
use clap::{Parser, Subcommand, ValueEnum};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(name = "todo", version, about = "Manage tasks from the terminal")]
pub struct Cli {
    #[arg(long, value_enum, global = true, default_value_t = OutputFormat::Table)]
    pub output: OutputFormat,

    #[command(subcommand)]
    pub command: TodoCommand,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
}

#[derive(Debug, Clone, PartialEq, Eq, Subcommand)]
pub enum TodoCommand {
    Add {
        title: String,
    },
    List {
        #[arg(long, value_enum, default_value_t = StatusArg::All)]
        status: StatusArg,
    },
    Done {
        id: Uuid,
    },
    Todo {
        id: Uuid,
    },
    Delete {
        id: Uuid,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum StatusArg {
    All,
    Todo,
    Done,
}

impl From<StatusArg> for ListTasksCommand {
    fn from(value: StatusArg) -> Self {
        Self::new(status_command_to_filter_task(value))
    }
}

pub fn status_command_to_filter_task(command: StatusArg) -> FilterTask {
    match command {
        StatusArg::All => FilterTask::All,
        StatusArg::Todo => FilterTask::Todo,
        StatusArg::Done => FilterTask::Done,
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, OutputFormat, StatusArg, TodoCommand};
    use clap::Parser;
    use uuid::Uuid;

    #[test]
    fn parses_add_command() {
        let cli = Cli::try_parse_from(["todo", "add", "Buy milk"]).expect("cli should parse add");

        assert_eq!(cli.output, OutputFormat::Table);

        assert_eq!(
            cli.command,
            TodoCommand::Add {
                title: "Buy milk".to_string(),
            }
        );
    }

    #[test]
    fn parses_list_command_with_default_status() {
        let cli = Cli::try_parse_from(["todo", "list"]).expect("cli should parse list");

        assert_eq!(cli.output, OutputFormat::Table);

        assert_eq!(
            cli.command,
            TodoCommand::List {
                status: StatusArg::All,
            }
        );
    }

    #[test]
    fn parses_list_command_with_explicit_status() {
        let cli = Cli::try_parse_from(["todo", "list", "--status", "done"])
            .expect("cli should parse list with status");

        assert_eq!(cli.output, OutputFormat::Table);

        assert_eq!(
            cli.command,
            TodoCommand::List {
                status: StatusArg::Done,
            }
        );
    }

    #[test]
    fn parses_done_command_with_uuid() {
        let id = Uuid::new_v4();
        let cli =
            Cli::try_parse_from(["todo", "done", &id.to_string()]).expect("cli should parse done");

        assert_eq!(cli.output, OutputFormat::Table);

        assert_eq!(cli.command, TodoCommand::Done { id });
    }

    #[test]
    fn parses_global_output_flag() {
        let cli = Cli::try_parse_from(["todo", "--output", "json", "list"])
            .expect("cli should parse global output");

        assert_eq!(cli.output, OutputFormat::Json);
        assert_eq!(
            cli.command,
            TodoCommand::List {
                status: StatusArg::All,
            }
        );
    }

    #[test]
    fn rejects_invalid_uuid_for_done_command() {
        let parsed = Cli::try_parse_from(["todo", "done", "not-a-uuid"]);

        assert!(parsed.is_err());
    }
}
