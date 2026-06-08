# state: task-list-crud
feature: task-list-crud
branch: feat/task-list-crud
phase: done
overall: done
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: T-6 done — TomlTaskListRepository + AT-8; all 12 tests green, clippy + fmt clean
next: feature complete — start the next feature with /sth-harness:add-spec (suggest: task-crud, then tui-shell)
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md
  on demand: ../../context/architecture.md · ../../context/testing.md · ../context/project.md
  skip: other features · whole context/

tasks:
  T-1 done  CreateTaskList + TaskList domain (non-empty name) (REQ-1, AT-1, AT-2)
  T-2 done  Enforce unique name on create               (REQ-2, AT-3)
  T-3 done  ListTaskLists usecase                        (REQ-3, AT-4)
  T-4 done  RenameTaskList (non-empty + unique + NotFound)(REQ-4, AT-5, AT-6)
  T-5 done  DeleteTaskList (NotFound)                    (REQ-5, AT-7)
  T-6 done  TomlTaskListRepository persists ./.shtask/   (REQ-6, AT-8)

watch-outs:
  - usecase tests use InMemoryTaskListRepository; only the TOML adapter touches the filesystem
  - uniqueness is a usecase rule via find_by_name, not a TaskList invariant
  - domain stays serde-free; the TOML adapter maps a DTO <-> TaskList at the boundary
  - SeqIdGenerator is deterministic (test-grade); a real IdGenerator + composition root land with the tui feature
  - no driving adapter yet — usecases are exercised by tests, not a CLI/TUI
