# ToDo CLI

A Rust command-line To-Do application focused on clean architecture and learning-by-building.

This project is intentionally being developed step by step with:
- Hexagonal Architecture (Ports and Adapters)
- Screaming Architecture (feature-first folders)
- Strong domain modeling and explicit error handling

This project is based on the beginner Rust challenge ideas shared by `CodeCrafters`:
https://codecrafters.io/blog/rust-projects

## Project Goals

- Add, list, complete, and delete tasks from the terminal
- Persist tasks to disk (JSON)
- Keep behavior predictable through domain rules
- Practice Rust fundamentals: file I/O, error handling, testing, and iterators

## Architecture

Design principles used:
- Domain is independent of infrastructure details.
- Use cases orchestrate ports and domain logic.
- Adapters implement I/O concerns (CLI, filesystem, serialization).

## Getting Started

### Prerequisites
- Rust stable toolchain
- Cargo

### Build / Check

```bash
cargo check
```

### Run tests

```bash
cargo test
```

## Roadmap (MVP)

1. Add task
2. List tasks (all / todo / done)
3. Mark task as done
4. Delete task
5. Persist tasks in local JSON file

## Why this project?

This repo is meant to be a practical Rust learning project with production-style architectural discipline from day one.
