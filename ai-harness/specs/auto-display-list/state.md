# state: auto-display-list
feature: auto-display-list
branch: claude-after-codex
phase: review
overall: review
updated: 2026-06-10
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: T-1..T-4 done; all 89 tests green; AT-1/AT-2/AT-3 written and passing
next: T-5 — manual smoke: cargo run, navigate lists with Up/Down, confirm right pane updates; enter with Right/Tab; navigate to empty list
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md design section
  on demand: ../../context/architecture.md
  skip: other features · whole context/

tasks:
  T-1 done  Add preview_tasks field to AppState                                            (REQ-1, AT-1, AT-3)
  T-2 done  Implement load_preview() helper + wire into select_next/select_previous/refresh (REQ-1, AT-1, AT-3)
  T-3 done  Update render_task_list to render preview in Lists mode                        (REQ-1, AT-1, AT-3)
  T-4 done  Write AT-1, AT-2, AT-3 tests                                                   (REQ-1, REQ-2, REQ-3)
  T-5 todo  Manual smoke test (DV-1)                                                       (REQ-1, REQ-2, REQ-3)

watch-outs:
  - preview shows without selection cursor (no >> marker, no highlight) — intentional read-only display
  - 3 existing tests updated: help_view, left_arrow_returns_to_lists, tab_returns_to_lists_from_tasks
  - "No task list open" still shows when lists vec is empty (no lists created yet)
