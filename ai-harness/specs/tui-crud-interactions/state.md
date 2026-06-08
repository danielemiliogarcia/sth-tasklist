# state: tui-crud-interactions
feature: tui-crud-interactions
branch: feat/tui-crud-interactions
phase: done
overall: done
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: validation status polished — TUI now maps typed use-case errors to user-facing text; 45 tests green, fmt/clippy/release/headless clean
next: feature complete — ready for review/merge
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md
  on demand: ../../context/architecture.md · ../../context/testing.md · ../tui-shell/spec.md · src/adapters/tui_app.rs · src/application/*
  skip: unrelated feature specs · whole context/

tasks:
  T-1 done Help/status/input state foundation + README keybindings (REQ-1, REQ-12, AT-1, AT-11)
  T-2 done List create flow in list mode (REQ-2, REQ-10, REQ-11, AT-2, AT-10)
  T-3 done List rename/delete flows with confirmation (REQ-3, REQ-4, REQ-10, REQ-11, AT-3, AT-4, AT-10)
  T-4 done Task selection/navigation in task mode (REQ-5, AT-5)
  T-5 done Task create/rename/delete flows (REQ-6, REQ-7, REQ-8, REQ-10, REQ-11, AT-6, AT-7, AT-8, AT-10)
  T-6 done Task completion flow + list completion refresh (REQ-9, AT-9)
  T-7 done Validation/cancel polish across edit/confirm modes (REQ-10, REQ-11, AT-10)
  T-8 done Manual terminal smoke or pty automation for CRUD ergonomics (REQ-1, REQ-2, REQ-3, REQ-4, REQ-5, REQ-6, REQ-7, REQ-8, REQ-9, REQ-10, REQ-11, DV-1)

watch-outs:
  - do not add new domain rules for this feature; existing use cases already enforce invariants
  - keep concrete TOML/UUID construction in main.rs; App tests should use InMemoryTaskListRepository and SeqIdGenerator
  - AppState is adapter state only; after mutation, refresh from the repository
  - task completion is one-way; do not invent an uncomplete shortcut in the TUI
  - TDD target is src/adapters/tui_app.rs module tests with ratatui TestBackend and in-memory repo
  - PTY smoke was run on an empty .shtask and left it empty after deleting the test list
  - selection must be visible without relying only on reverse-video terminal styling
  - Up/Down wrap around when there is more than one selectable list/task
  - TUI status messages should stay user-facing; do not expose raw Rust enum names
