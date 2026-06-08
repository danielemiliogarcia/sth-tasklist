# state: panel-navigation
feature: panel-navigation
branch: feat/panel-navigation
phase: tasks
overall: todo
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: spec and state created; no code written yet
next: add AT-1 test (Right opens tasks panel) in tui_app.rs tests, run cargo test to confirm red, then add KeyCode::Right arm in handle_normal_key
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md acceptance + design sections
  on demand: src/adapters/tui_app.rs (handle_normal_key + render_help)
  skip: other features · whole context/ · domain layer

tasks:
  T-1 todo write AT-1: Right opens tasks panel (REQ-1, AT-1)
  T-2 todo write AT-2: Left returns to lists (REQ-2, AT-2)
  T-3 todo write AT-3: Right is equivalent to Enter (REQ-1, REQ-3, AT-3)
  T-4 todo write AT-4: Left is equivalent to Esc (REQ-2, REQ-3, AT-4)
  T-5 todo write AT-5: Right is no-op when lists is empty (REQ-1, AT-5)
  T-6 todo add KeyCode::Right arm in handle_normal_key (REQ-1, AT-1, AT-3, AT-5)
  T-7 todo add KeyCode::Left arm in handle_normal_key (REQ-2, AT-2, AT-4)
  T-8 todo update render_help list-mode items to include "Right open tasks" (REQ-1)
  T-9 todo update render_help task-mode items to include "Left return to lists" (REQ-2)
  T-10 todo run cargo test — all AT-* green; run DV-1 manual smoke (REQ-1, REQ-2, REQ-3, DV-1)

watch-outs:
  - Right/Left must be added inside the Interaction::None branch only; the existing dispatch in handle_key already gates them away from Editing/Confirming/Help states
  - Left arm logic is three lines (mode = Lists, tasks.clear(), selected_task = 0) — copy exactly from Esc arm, do not refactor Esc
  - Right arm should call self.open_tasks()? not inline the logic, to stay DRY with Enter
