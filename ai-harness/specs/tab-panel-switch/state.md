# state: tab-panel-switch
feature: tab-panel-switch
branch: feat/tab-panel-switch
phase: done
overall: done
updated: 2026-06-08
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: all 7 AT tests green; Tab/BackTab arm added; help text updated; 73 tests pass
next: none — spec complete
blocker: none

read budget:
  now: this file
  on demand: src/adapters/tui_app.rs
  skip: other features · whole context/

tasks:
  T-1 done write failing tests AT-1..AT-5 (and AT-6, AT-7 for REQ-4) using TestBackend (REQ-1, REQ-2, REQ-3, REQ-4, AT-1, AT-2, AT-3, AT-4, AT-5, AT-6, AT-7)
  T-2 done add KeyCode::Tab match arm in handle_normal_key — forward (Lists→Tasks) and backward (Tasks→Lists) (REQ-1, REQ-2, AT-1, AT-2, AT-3, AT-4)
  T-3 done add KeyCode::BackTab match arm in handle_normal_key for Shift+Tab reverse cycling (REQ-4, AT-6, AT-7)
  T-4 done update render_help to mention Tab in both modes (REQ-3, AT-5)
  T-5 done run cargo test and cargo clippy; confirm all existing tests still pass (REQ-2)

watch-outs:
  - Tab and BackTab share one arm (both toggle; in 2-panel app forward/backward are symmetric)
  - help text updated to "Enter/Right/Tab open tasks" and "Esc/Left/Tab return to lists"
  - existing help_view test updated to check new string
