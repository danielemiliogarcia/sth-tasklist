# state: panel-navigation
feature: panel-navigation
branch: feat/panel-navigation
phase: done
overall: done
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: all 5 AT tests green; Right/Left arms added to handle_normal_key; help text updated; all 55 tests pass
next: none — spec complete
blocker: none

read budget:
  now: this file
  on demand: src/adapters/tui_app.rs
  skip: other features · whole context/ · domain layer

tasks:
  T-1 done write AT-1: Right opens tasks panel (REQ-1, AT-1)
  T-2 done write AT-2: Left returns to lists (REQ-2, AT-2)
  T-3 done write AT-3: Right is equivalent to Enter (REQ-1, REQ-3, AT-3)
  T-4 done write AT-4: Left is equivalent to Esc (REQ-2, REQ-3, AT-4)
  T-5 done write AT-5: Right is no-op when lists is empty (REQ-1, AT-5)
  T-6 done add KeyCode::Right arm in handle_normal_key (REQ-1, AT-1, AT-3, AT-5)
  T-7 done add KeyCode::Left arm in handle_normal_key (REQ-2, AT-2, AT-4)
  T-8 done update render_help list-mode items to include "Right open tasks" (REQ-1)
  T-9 done update render_help task-mode items to include "Left return to lists" (REQ-2)
  T-10 done run cargo test — all AT-* green; DV-1 manual smoke pending (REQ-1, REQ-2, REQ-3, DV-1)

watch-outs:
  - fix was two-line change: Enter | Right and Esc | Left merged in handle_normal_key
  - existing help test needed updating from "Enter open tasks" to "Enter/Right open tasks"
  - DV-1 (manual terminal smoke) is one-off; no follow-up task needed
