# spec: task-crud
intent: add, list, edit, delete, and complete tasks inside a list; a list auto-completes when all its tasks are done
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): the system shall add a task to a list from a non-empty title, rejecting empty/whitespace (EmptyTitle)
  REQ-2 (must): the system shall reject adding a task whose title already exists in the same list (DuplicateTitle, case-sensitive)
  REQ-3 (must): the system shall return a list's tasks in insertion order
  REQ-4 (must): the system shall rename a task, applying the non-empty + unique-in-list rules (TaskNotFound if absent)
  REQ-5 (must): the system shall delete a task from a list (TaskNotFound if absent)
  REQ-6 (must): the system shall mark a task completed, one-way; re-completing an already-completed task is a no-op (no error)
  REQ-7 (must): a list is completed iff it has at least one task and every task is completed (derived automatically)
  REQ-8 (should): an operation targeting a missing list reports TaskListNotFound

scope:
  in: CRUD + one-way completion of tasks within a list; derived list completion; persistence of tasks in the list's TOML file
  out: reordering tasks; un-completing a task; due dates/priorities/notes; TUI rendering; multi-list moves
assumptions:
  - tasks are addressed by their (unique) title within a list — no separate task id
  - title comparison is case-sensitive and exact (empty check trims whitespace)
  - TaskList is the aggregate root that owns its tasks and enforces uniqueness + completion
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (unit, red):
    Given a list / When add a blank-title task / Then raise EmptyTitle and no task is added
  AT-2 covers REQ-1 (unit):
    Given a list / When add "milk" / Then the list has one incomplete task "milk"
  AT-3 covers REQ-2 (unit, red):
    Given a list already holding "milk" / When add "milk" / Then raise DuplicateTitle
  AT-4 covers REQ-3 (unit):
    Given tasks "a" then "b" are added / When read tasks / Then ["a","b"] in that order
  AT-5 covers REQ-4 (unit):
    Given a task "milk" / When rename it to "bread" / Then its title is "bread"
  AT-6 covers REQ-4 (unit, red):
    Given tasks "milk" and "bread" / When rename "bread" to "milk" / Then raise DuplicateTitle
  AT-7 covers REQ-4 (unit, red):
    Given a list / When rename a missing task / Then raise TaskNotFound
  AT-8 covers REQ-5 (unit):
    Given a task "milk" / When delete "milk" / Then it is no longer present
  AT-9 covers REQ-5 (unit, red):
    Given a list / When delete a missing task / Then raise TaskNotFound
  AT-10 covers REQ-6 (unit):
    Given an incomplete task "milk" / When complete "milk" / Then it is completed
  AT-11 covers REQ-6 (unit):
    Given a completed task "milk" / When complete "milk" again / Then it stays completed with no error
  AT-12 covers REQ-7 (unit, red):
    Given a list with one completed and one incomplete task / Then it is not completed; When the other is completed / Then it is completed
  AT-13 covers REQ-7 (unit):
    Given a list with zero tasks / Then it is not completed
  AT-14 covers REQ-8 (unit, red):
    Given no list with the given id / When add a task via the use case / Then raise TaskListNotFound
  AT-15 covers REQ-3, REQ-6 (integration):
    Given a list with a completed task saved via the TOML repository / When a fresh repository reloads it / Then the task and its completed flag persist

design:
  domain:
    Task (title, completed; invariant: title non-empty/non-whitespace; new tasks start incomplete)
    TaskList (aggregate root) extended to own tasks: add_task, rename_task, remove_task, complete_task, tasks(), is_completed()
    invariants on the aggregate: titles unique within the list; is_completed() == (!tasks.is_empty() && all completed)
    TaskError (feature-local domain error): EmptyTitle, DuplicateTitle, NotFound — kept separate from DomainError so task-list-crud's error mappings stay untouched
  ports: reuse TaskListRepository (by_id, save, all) from task-list-crud — no new port
  adapters:
    driven: extend TomlTaskListRepository's DTO with tasks (TaskDto { title, completed }), serde default empty -> backward compatible with existing list files; rehydrate tasks on load
    driving: (deferred — the tui feature wires these usecases)
  usecases (each: load list by_id -> TaskListNotFound; mutate aggregate -> map TaskError; save):
    AddTask, RenameTask, DeleteTask, CompleteTask, ListTasks (load -> return tasks + is_completed)
    shared usecase error TaskCommandError { ListNotFound, EmptyTitle, DuplicateTitle, TaskNotFound, Repo } with From<TaskError>
  feature-local conventions:
    - tasks keyed by unique title within a list (no task id); rename updates that key
    - new domain type Task + TaskError; usecase error TaskCommandError
    - TaskList is now the aggregate root owning tasks; uniqueness + completion are aggregate invariants
    - TOML DTO gains `tasks` (serde default) — additive, backward compatible with task-list-crud
