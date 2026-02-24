# ToDo CLI

A Rust command-line To-Do application built with a clean, layered architecture.

Challenge context: see `CHALLENGE.md`.

Architecture details: see `docs/architecture.md` and `docs/step-1-mvp.md`.

## Quickstart

```bash
# Add a task
cargo run -- add "Buy milk"

# List tasks
cargo run -- list

# Mark a task as done
cargo run -- done <task-uuid>
```

## Features

- Add tasks
- List tasks (`all`, `todo`, `done`)
- Mark tasks as `done`
- Mark tasks back to `todo`
- Delete tasks
- Persist tasks to a local JSON file

## Requirements

- Rust stable toolchain
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

You can run the app with `cargo run` during development:

```bash
cargo run -- <command>
```

Or run the built binary directly:

```bash
./target/release/todo-cli <command>
```

### Selecting output format

The default output format is `table`.

Use `--output json` to get JSON instead:

```bash
cargo run -- --output json list
cargo run -- --output json add "Buy milk"
```

### Commands

#### Add a task

```bash
cargo run -- add "Buy milk"
```

#### List tasks

List all tasks (default):

```bash
cargo run -- list
```

List by status:

```bash
cargo run -- list --status todo
cargo run -- list --status done
```

#### Mark task as done

```bash
cargo run -- done <task-uuid>
```

#### Mark task back to todo

```bash
cargo run -- todo <task-uuid>
```

#### Delete a task

```bash
cargo run -- delete <task-uuid>
```

## Output behavior

- Default format is table for all commands.
- Use `--output json` for machine-readable output.
- In table mode:
  - `add`, `list`, `done`, `todo` render tabular rows.
  - `delete` prints either `deleted <id>` or `task <id> not found`.
- In JSON mode:
  - `add`, `list`, `done`, `todo` return serialized task payloads.
  - `delete` returns a structured object with `id`, `deleted`, and `message`.

Errors are printed to `stderr`, and the process exits with code `1`.

## Data storage

Tasks are persisted in a JSON file managed through the platform-specific project config directory (via the `directories` crate), under a `data/tasks.json` path.

## Architecture

This project follows:

- Hexagonal Architecture (Ports and Adapters)
- Screaming Architecture (feature-first folders)

High-level design:

- `domain`: business rules and task state transitions
- `application/use_cases`: orchestration of domain + repository ports
- `ports`: interfaces for driven adapters
- `adapters`: CLI and persistence implementations

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
