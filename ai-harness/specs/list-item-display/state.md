# state: list-item-display
feature: list-item-display
branch: claude-after-codex
phase: done
overall: done
updated: 2026-06-10
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: T-3/T-4 fixed — "  " prefix in preview_task_items(); AT-2/AT-3 rewritten to test cross-mode column; 93 tests green
next: none — spec complete
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md design section
  on demand: ../../context/architecture.md
  skip: other features · whole context/

tasks:
  T-1 done  Change "pending" to "☐" in render_lists                              (REQ-1, AT-1)
  T-2 done  Remove custom >> marker from render_lists and task_items()            (REQ-2, AT-2, AT-3)
  T-3 done  Fix preview_task_items() — add "  " prefix to align with Tasks mode  (REQ-2, AT-2)
  T-4 done  Fix AT-2/AT-3 tests to cover cross-mode column consistency            (REQ-2)

watch-outs:
  - "  " prefix (2 spaces) in preview items matches ratatui highlight_symbol width ("> " = 2 chars)
  - AT-2 tests preview→Tasks column; AT-3 tests preview vs non-selected column in Tasks mode
