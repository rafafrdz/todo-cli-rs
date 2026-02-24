# Architecture

This project follows a Hexagonal + Screaming Architecture style for a Rust CLI application.

## Layer Boundaries

- `domain`: business rules, entities, and invariant validation.
- `application/use_cases`: orchestration (`find -> transition -> save`) through ports.
- `ports`: trait contracts used by application.
- `adapters`: I/O and infrastructure (`cli`, `persistence`).

Rules:

- Domain does not know about filesystem, serialization, or clap.
- Application depends on ports and domain, not concrete adapters.
- Adapters implement ports and convert external data/errors.

## Domain Immutability Checklist

- [x] No `&mut self` in domain transitions.
- [x] Transitions are pure (`mark_done`, `mark_todo`, `edit_title`) and side-effect free.
- [x] Validation lives in domain and returns `DomainError`.
- [x] Fields are private; updates happen through domain methods only.
- [x] `created_at` is preserved.
- [x] `modified_at` updates only on successful transitions.
- [x] Style is consistent (immutable transitions, no mixed mutation model).
- [x] Use cases orchestrate, domain decides.

Recommended domain conventions:

- Transition signatures: `self -> DomainResult<Self>`.
- Getters return copies/references (`Uuid`, `TaskStatus`, `&str`, `DateTime<Utc>`).

## Error Model by Layer

- `DomainError`: business rule violations.
- `RepoError`: persistence/infrastructure failures.
- `ApplicationError`: wraps domain and repository errors.
- `CliError`: wraps application + output serialization errors.

Guidelines:

- Prefer typed errors over panics.
- Keep error context actionable for end users.
- Use transparent forwarding when wrapping lower-layer errors.

## Persistence Strategy

The active repository adapter is JSON file based.

- Repository type: `JsonFileTaskRepository`
- File storage: platform config directory + `data/tasks.json`
- Supports `save`, `list`, `find_by_id`, and `delete`
- Invalid JSON returns an explicit repository error

## CLI Contract

Supported commands:

- `add <title>`
- `list [--status <all|todo|done>]`
- `done <id>`
- `todo <id>`
- `delete <id>`

Output mode:

- `--output table|json` (default: `table`)

## Testing Expectations

- Unit tests close to implementation (`#[cfg(test)]`).
- Cover happy and invalid paths for domain transitions and repositories.
- Keep test names behavior-focused.

## Refactoring Guardrails

- Keep adapters thin: parse/format + call use cases.
- Keep use cases explicit and small.
- Keep domain logic deterministic and side-effect free.
