# state: tab-panel-switch
feature: tab-panel-switch
branch: feat/tab-panel-switch
phase: tasks
overall: todo
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: spec and state files created; ready to begin implementation
next: write failing adapter tests for AT-1 and AT-2 (Tab forward/backward) using TestBackend in src/adapters/tui_app.rs tests module

blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md acceptance + design sections
  on demand: ../../context/architecture.md · src/adapters/tui_app.rs (handle_normal_key, render_help)
  skip: other features · whole context/

tasks:
  T-1 todo write failing tests AT-1..AT-5 (and AT-6, AT-7 for REQ-4) using TestBackend (REQ-1, REQ-2, REQ-3, REQ-4, AT-1, AT-2, AT-3, AT-4, AT-5, AT-6, AT-7)
  T-2 todo add KeyCode::Tab match arm in handle_normal_key — forward (Lists→Tasks) and backward (Tasks→Lists) (REQ-1, REQ-2, AT-1, AT-2, AT-3, AT-4)
  T-3 todo add KeyCode::BackTab match arm in handle_normal_key for Shift+Tab reverse cycling (REQ-4, AT-6, AT-7)
  T-4 todo update render_help to mention Tab in both modes (REQ-3, AT-5)
  T-5 todo run cargo test and cargo clippy; confirm all existing tests still pass (REQ-2)

watch-outs:
  - KeyCode::BackTab is the crossterm representation of Shift+Tab; verify the import is already in scope before adding the arm
  - Tab in Lists mode must call open_tasks() (which guards against empty lists) rather than setting mode directly, to preserve the no-op-on-empty behavior
  - help text format must be consistent with the existing panel-navigation help lines added by that feature
