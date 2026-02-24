# Challenge

This repository implements **Project #1 (To-Do CLI)** from the CodeCrafters Rust project ideas list:

- Source: https://codecrafters.io/blog/rust-projects

## What the challenge is about

Build a command-line To-Do application in Rust that is useful enough to run daily while practicing core Rust skills and software design.

At minimum, the challenge includes:

- Adding tasks
- Listing tasks
- Marking tasks as done
- Deleting tasks
- Persisting data to local storage

## Why this challenge

The project is intentionally small in product scope, but rich in engineering practice:

- Domain modeling and invariants
- Explicit error handling across layers
- CLI ergonomics and argument parsing
- Persistence and data serialization
- Automated tests and refactoring discipline

## This repository's interpretation

This implementation goes beyond a single-file script and applies:

- Hexagonal Architecture (Ports and Adapters)
- Screaming Architecture (feature-first folder layout)
- Immutable domain transitions

Current behavior exposed by the CLI:

- `add`, `list`, `done`, `todo`, `delete`
- output selection via `--output table|json` (default: `table`)

## Success criteria for the challenge

The challenge can be considered complete when:

- All core commands behave correctly
- Data persists reliably between runs
- Domain rules are enforced consistently
- Error messages are actionable for CLI users
- Tests validate both happy paths and failure paths

## Notes

- The canonical challenge inspiration is the CodeCrafters article linked above.
- This repository may evolve beyond the original challenge constraints as a learning and architecture playground.
- For implementation details, see `README.md`, `docs/architecture.md`, and `docs/step-1-mvp.md`.
