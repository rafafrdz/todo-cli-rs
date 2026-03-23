# ToDo CLI

A Rust terminal To-Do application built with a clean, layered architecture and a TUI (Terminal User Interface) powered by [ratatui](https://ratatui.rs/).

Challenge context: see `CHALLENGE.md`.

Architecture details: see `docs/architecture.md` and `docs/step-1-mvp.md`.

## Quickstart

```bash
cargo run
```

The TUI launches immediately. Use keyboard shortcuts to manage your tasks.

## Features

- Add tasks
- Edit task titles
- List tasks with filters (`all`, `todo`, `done`)
- Toggle task status between `todo` and `done`
- Delete tasks with confirmation
- Persist tasks to a local JSON file
- Interactive TUI with modal input and status feedback

## Requirements

- Rust stable toolchain (edition 2024)
- Cargo

## Build and Development

Run commands from the repository root.

### Fast checks

```bash
cargo check
cargo test
```

### Build binary

```bash
cargo build --release
```

The binary will be available at:

```bash
target/release/todo-cli
```

### Quality checks

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
```

## Usage

### Launch

```bash
cargo run
```

Or run the built binary directly:

```bash
./target/release/todo-cli
```

### Keyboard shortcuts

#### Normal mode

| Key       | Action                                    |
|-----------|-------------------------------------------|
| `a`       | Add a new task (opens input popup)        |
| `e`       | Edit selected task title (opens input popup) |
| `d`       | Delete selected task (asks confirmation)  |
| `x`       | Toggle selected task status (todo/done)   |
| `f`       | Cycle filter: All -> Done -> Todo -> All  |
| `j` / `Down`  | Select next task                     |
| `k` / `Up`    | Select previous task                 |
| `q`       | Quit                                      |

#### Adding / Editing mode

| Key         | Action              |
|-------------|---------------------|
| `Enter`     | Confirm input       |
| `Esc`       | Cancel              |
| `Backspace` | Delete character    |
| Any char    | Append to input     |

#### Confirm Delete mode

| Key           | Action          |
|---------------|-----------------|
| `y` / `Enter` | Confirm delete |
| `n` / `Esc`   | Cancel         |

## Data storage

Tasks are persisted in a JSON file managed through the platform-specific project config directory (via the `directories` crate), under a `data/tasks.json` path.

## Architecture

This project follows:

- **Hexagonal Architecture** (Ports and Adapters)
- **Screaming Architecture** (feature-first folders)

### Layer overview

```
Adapters (TUI, Persistence) --> Application (Use Cases) --> Ports (Traits) --> Domain (Entities)
```

| Layer                  | Responsibility                                         |
|------------------------|--------------------------------------------------------|
| `domain`               | Business rules, entities, and task state transitions   |
| `application/use_cases`| Orchestration of domain logic + repository ports       |
| `ports`                | Trait contracts for driven adapters                    |
| `adapters/tui`         | Terminal UI (ratatui + crossterm)                      |
| `adapters/persistence` | In-memory and JSON file repository implementations     |

### Use cases

| Use Case       | Description                                    |
|----------------|------------------------------------------------|
| `AddTask`      | Create a new task with a title                 |
| `EditTask`     | Change the title of an existing task           |
| `ListTasks`    | List tasks with optional status filter         |
| `MarkTaskDone` | Transition a task from `Todo` to `Done`        |
| `MarkTaskTodo` | Transition a task from `Done` to `Todo`        |
| `DeleteTask`   | Remove a task by ID                            |

### Key design principles

- **Immutable domain transitions**: state changes consume `self` and return `DomainResult<Self>`.
- **Find-transition-save pattern**: use cases orchestrate `repo.find -> domain_method -> repo.save`.
- **Typed errors per layer**: `DomainError`, `RepoError`, `ApplicationError`, `TuiError`.
- **TEA pattern in TUI**: Model (`App`), View (`draw`), Update (`handle_events`).

## Running a single test

Useful during iteration:

```bash
cargo test save_and_find_by_id_returns_task
cargo test tasks::adapters::persistence::in_memory_task_repository::tests::save_and_find_by_id_returns_task
cargo test --lib in_memory_task_repository
```

With output visible:

```bash
cargo test <pattern> -- --nocapture
```

## Dependencies

| Crate        | Purpose                                |
|--------------|----------------------------------------|
| `ratatui`    | TUI framework (widgets, layout)        |
| `crossterm`  | Terminal backend (raw mode, events)    |
| `serde`      | Serialization / deserialization        |
| `serde_json` | JSON persistence format                |
| `chrono`     | Date/time handling                     |
| `thiserror`  | Error derive macros                    |
| `uuid`       | Unique task identifiers (v4)           |
| `directories`| Platform-specific config paths         |
| `tempfile`   | Temporary directories for tests        |
