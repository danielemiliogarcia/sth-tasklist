# state: hotkey-bar
feature: hotkey-bar
branch: codex-continue
phase: done
overall: done
updated: 2026-06-08
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: all 5 AT tests green; layout extended to 3 slices + render_hotkeys() added; 86 tests pass
next: none — spec complete
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md design section
  on demand: ../../context/architecture.md · src/adapters/tui_app.rs (render fn and surrounding methods)
  skip: other features · whole context/

tasks:
  T-1 todo  extend render() layout to three slices + call render_hotkeys (REQ-1, REQ-6, AT-1)
  T-2 todo  implement render_hotkeys: Lists/None arm (REQ-2, AT-2)
  T-3 todo  implement render_hotkeys: Tasks/None arm (REQ-3, AT-3)
  T-4 todo  implement render_hotkeys: Editing and Confirming arms + muted style (REQ-4, REQ-5, REQ-7, AT-4, AT-5)
  T-5 todo  manual DV-1 check: run app, verify bar visible and styled (DV-1)

watch-outs:
  - all changes are confined to src/adapters/tui_app.rs — no domain, port, or use-case files should be touched
  - the existing status_area Constraint::Length(3) must remain unchanged; only a new Constraint::Length(1) is added as the third slice
  - Help interaction (Interaction::Help) is not listed in REQ-2..5; handle it explicitly or fall through to a sensible default to avoid a match panic
  - ratatui Layout::areas() destructs into an array by count — changing from 2 to 3 slices requires updating the destructuring pattern from [main_area, status_area] to [main_area, status_area, hotkey_area]
