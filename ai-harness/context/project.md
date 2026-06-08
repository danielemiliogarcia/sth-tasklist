# Project Context

This file gives an agent the stable, high-level facts about the project that do
not belong to any single feature: what it is, what it is for, and where things
live. It is part of the minimal context a session may load.

---

## Reference documents

- Root [`README.md`](../../README.md) — what the app is, how to build/run/test.
- _Architecture / ADRs: none yet — capture decisions under `../decisions/` as they arise._

## What this project is

A terminal (TUI) task manager. The user can CRUD task lists and CRUD tasks
inside those lists, mark tasks completed, and when every task in a list is
completed the list itself is marked completed — all from the shell. Solves the
problem of organizing tasks without leaving the terminal.

**Application type:** CLI (TUI)

## Who it is for

The developer themselves (and shell-centric users) who want to organize tasks
without leaving the terminal.

## Goals and non-goals

- **Goals:** fast keyboard-driven TUI; full CRUD of lists and tasks; automatic
  list-completion when all its tasks are done; durable persistence to `.toml`
  files; idiomatic Rust (clippy-clean, rustfmt).
- **Non-goals:** no GUI, no web/server, no network sync, no multi-user, no
  database engine (TOML files only).

## Where the code lives

```text
.
├── ai-harness/        # this harness (process, specs, tasks, state)
├── src/               # application source (Rust crate)
│   ├── domain/        # Task, TaskList entities + invariants — pure, no IO
│   ├── application/   # use cases + port interfaces
│   └── adapters/      # driven (TOML repo) + driving (TUI) implementations
└── tests/             # integration tests
```

## How to build, run, and test

```text
build:  cargo build
run:    cargo run
test:   cargo test
lint:   cargo clippy --all-targets -- -D warnings && cargo fmt --check
```

## Tech stack and key constraints

- Language / runtime: Rust (stable, 2021 edition).
- Frameworks / libraries: `ratatui` + `crossterm` (TUI); `serde` + `toml`
  (persistence).
- External systems: none — state persists to local `.toml` files.
- Hard constraints: terminal-only; TOML is the sole storage format; follow Rust
  conventions (clippy-clean, rustfmt-formatted).

## Glossary (domain language)

| Term | Meaning |
|------|---------|
| Task | A single unit of work; has a title and a completed flag. |
| Task list | A named collection of tasks. |
| Completed (task) | A task the user has marked done. |
| Completed (list) | A list whose every task is completed (derived, automatic). |

---

Until customized, treat unknown items here as **open questions** and confirm with
the human rather than assuming.
