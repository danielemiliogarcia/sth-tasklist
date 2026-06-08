# state: uncomplete-task
feature: uncomplete-task
branch: codex-continue
phase: done
overall: done
updated: 2026-06-08
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: all 6 AT tests green; domain + use case + TUI toggle + text updates done; 82 tests pass
next: none — spec complete
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md design section
  on demand: ../../context/architecture.md · src/domain/task.rs · src/domain/task_list.rs
  skip: other features · whole context/

tasks:
  T-1 todo  Task::mark_uncompleted + TaskList::uncomplete_task + unit tests  (REQ-1, REQ-2, AT-1, AT-2, AT-3)
  T-2 todo  UncompleteTask use case + unit test                               (REQ-3, AT-4)
  T-3 todo  TUI toggle logic (Space branch + method rename/update)            (REQ-4, REQ-5, AT-5)
  T-4 todo  Status bar + help screen text update                              (REQ-6, AT-6)

watch-outs:
  - Task::mark_completed is pub(crate); mark_uncompleted must match that visibility
  - complete_task on TaskList is currently documented as one-way; drop that wording when adding uncomplete_task
  - The existing Space test at tui_app.rs:1064 asserts complete behaviour; it must remain green after the toggle change (incomplete task -> Space -> complete is still the happy path)
  - TomlTaskListRepository persists the completed bool via serde; no migration needed as false is the default for new tasks
