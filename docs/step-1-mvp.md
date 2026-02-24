# Step 1 - MVP (Hexagonal + Screaming)

This document now reflects the implemented result of Step 1.

## 1) Context and Goal

- Project: Rust To-Do CLI
- Target architecture: Hexagonal + Screaming Architecture
- MVP goal: manage tasks from terminal with local persistence
- Out of scope for now: cloud sync, multi-user support, advanced TUI

## 2) Implemented Use Cases (Input Ports)

- `AddTask`
  - Input: `title`
  - Rule: title cannot be blank
  - Result: task created with unique ID and `Todo` status
- `ListTasks`
  - Input: optional status filter (`all|todo|done`)
  - Result: matching tasks
- `MarkTaskDone`
  - Input: `id`
  - Rule: task must exist and transition must be valid
  - Result: status becomes `Done`
- `MarkTaskTodo`
  - Input: `id`
  - Rule: task must exist and transition must be valid
  - Result: status becomes `Todo`
- `DeleteTask`
  - Input: `id`
  - Result: boolean deleted/not-found outcome

## 3) Domain Model (Implemented)

- Entity: `Task`
  - `id: Uuid`
  - `title: String`
  - `status: TaskStatus` (`Todo | Done`)
  - `created_at: DateTime<Utc>`
  - `modified_at: DateTime<Utc>`

Domain transitions are immutable and return `DomainResult<Self>`.

## 4) Business Rules

- Empty titles are rejected.
- IDs are unique (`Uuid`).
- Invalid transitions return explicit domain errors.
- `created_at` remains stable across transitions.
- `modified_at` updates on successful transitions.

## 5) Hexagonal Design (Ports and Adapters)

- Input adapter:
  - CLI (clap) parses arguments and invokes use cases
- Output port:
  - `TaskRepository` (`save`, `list`, `find_by_id`, `delete`)
- Output adapter:
  - JSON file repository (`JsonFileTaskRepository`)

## 6) Screaming Structure

```text
src/
  tasks/
    domain/
    application/
      use_cases/
    ports/
      outputs/
    adapters/
      cli/
      persistence/
```

## 7) CLI Contract (Current)

- `todo-cli add "Buy milk"`
- `todo-cli list`
- `todo-cli list --status todo`
- `todo-cli done <uuid>`
- `todo-cli todo <uuid>`
- `todo-cli delete <uuid>`

Output mode:

- `--output table|json`
- default output: `table`

## 8) Persistence (Current)

- JSON file stored in platform config directory under `data/tasks.json`
- Missing file is treated as empty state
- Mutating operations persist immediately
- Invalid JSON surfaces explicit repository errors

## 9) Error Taxonomy

- `DomainError`: validation and transition rules
- `RepoError`: persistence/read/write/parse failures
- `ApplicationError`: domain + repository orchestration errors
- `CliError`: application + serialization for CLI output

## 10) Definition of Done (Step 1)

- [x] MVP use cases implemented
- [x] Domain rules enforced
- [x] Ports and adapters defined and wired
- [x] CLI adapter implemented with clap
- [x] JSON file persistence implemented
- [x] Command contract documented
- [x] Output format toggle (`table` default, `json` optional)
- [x] Automated tests passing
