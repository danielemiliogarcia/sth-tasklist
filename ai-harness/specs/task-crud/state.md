# state: task-crud
feature: task-crud
branch: feat/task-crud
phase: done
overall: done
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: T-7 done — TOML DTO carries tasks (backward compatible) + AT-15; 29 tests green, clippy + fmt clean
next: feature complete — next feature is tui-shell (driving adapter + composition root + real IdGenerator) via /sth-harness:add-spec
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md
  on demand: ../../context/architecture.md · ../../context/testing.md · ../context/project.md
  skip: other features · whole context/

tasks:
  T-1 done  Task VO + TaskList.add_task (non-empty + unique) (REQ-1, REQ-2, AT-1, AT-2, AT-3)
  T-2 done  AddTask + ListTasks usecases (TaskListNotFound)  (REQ-3, REQ-8, AT-4, AT-14)
  T-3 done  RenameTask (non-empty + unique + NotFound)       (REQ-4, AT-5, AT-6, AT-7)
  T-4 done  DeleteTask (NotFound)                            (REQ-5, AT-8, AT-9)
  T-5 done  CompleteTask one-way + idempotent                (REQ-6, AT-10, AT-11)
  T-6 done  is_completed derivation incl empty-list          (REQ-7, AT-12, AT-13)
  T-7 done  extend TOML DTO with tasks (round-trip)          (REQ-3, REQ-6, AT-15)

watch-outs:
  - TaskList is the aggregate root owning tasks; uniqueness + completion are aggregate invariants
  - TOML DTO `tasks` uses serde default -> backward compatible with task-list-crud list files
  - tasks keyed by unique title; rename changes the key; one-way completion (no uncomplete)
  - empty list is NOT completed (is_completed requires >=1 task)
  - usecases still lack a driving adapter; tui-shell will add the TUI + real IdGenerator + composition root
