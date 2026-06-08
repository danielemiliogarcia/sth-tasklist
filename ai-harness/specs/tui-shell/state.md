# state: tui-shell
feature: tui-shell
branch: feat/tui-shell
phase: done
overall: done
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: T-7 done — TestBackend smoke covers list/task rendering and navigation; 32 tests green, fmt/clippy/release build clean
next: feature complete — ready for review/merge
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md
  on demand: ../../context/architecture.md · ../../context/project.md
  skip: other features · whole context/

tasks:
  T-1 done UuidIdGenerator driven adapter (REQ-1, AT-1)
  T-2 done Composition root — wire usecases in main.rs (REQ-2, DV-1)
  T-3 done App skeleton — launch, render lists panel, q to quit (REQ-3, REQ-4, DV-2, DV-3)
  T-4 done Navigation — ↑/↓ + selection highlight (REQ-5, DV-4)
  T-5 done Task panel — Enter opens tasks view, Esc returns (REQ-6, DV-5)
  T-6 done --headless integration test (follow-up from DV-1)
  T-7 done TestBackend smoke test (follow-up from DV-2/3/4/5)

watch-outs:
  - ratatui + crossterm are now TUI dependencies in Cargo.toml
  - composition root is the only place real adapters are constructed; all tests stay on InMemory/Seq
  - AppState lives in the adapter layer (not domain); rebuild from repo after any mutation
  - DV-n manual terminal checks were run for T-3/T-4/T-5 using ignored ./.shtask sample data
  - T-6 + T-7 automated follow-ups are done
