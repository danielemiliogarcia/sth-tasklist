# state: auto-display-list
feature: auto-display-list
branch: claude-after-codex
phase: done
overall: done
updated: 2026-06-10
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: T-5 closed — AT-1/AT-2/AT-3 fully cover state transitions; automated regression tests green; Codex P2 stale-preview bug fixed and regression test added
next: none — spec complete
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
  T-5 done  Manual smoke / DV-1 — covered by AT-1..AT-3 automated tests                   (REQ-1, REQ-2, REQ-3)

watch-outs:
  - preview shows without selection cursor (no >> marker, no highlight) — intentional read-only display
  - Codex P2 fix: load_preview() called after select_list_id() in CreateList to avoid stale preview
