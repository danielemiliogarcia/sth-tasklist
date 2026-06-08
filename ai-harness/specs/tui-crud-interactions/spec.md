# spec: tui-crud-interactions
intent: make task lists and tasks fully manageable from the ratatui UI without editing .shtask files
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): the TUI shall expose in-app help for the active mode, including create, rename, delete, complete, navigation, cancel, and quit keys
  REQ-2 (must): from list mode, the user shall create a task list by entering a non-empty name; the new list is persisted, visible, and selected
  REQ-3 (must): from list mode, the user shall rename the selected task list by entering a non-empty new name; existing tasks are preserved
  REQ-4 (must): from list mode, the user shall delete the selected task list after an explicit confirmation; the list is removed from persistence and the selection remains valid
  REQ-5 (must): in task mode, the user shall navigate tasks with Up/Down, with the selected task visually highlighted independently from the selected list
  REQ-6 (must): in task mode, the user shall create a task in the selected list by entering a non-empty title; the task is persisted and visible
  REQ-7 (must): in task mode, the user shall rename the selected task by entering a non-empty new title; the renamed task is persisted and visible
  REQ-8 (must): in task mode, the user shall delete the selected task after an explicit confirmation; the task is removed from persistence and the selection remains valid
  REQ-9 (must): in task mode, the user shall mark the selected task completed; task completion is persisted and the list completion badge refreshes when all tasks are complete
  REQ-10 (must): invalid inputs and rejected mutations, including empty names/titles and duplicates, shall show a visible status message and leave persisted data unchanged
  REQ-11 (must): Esc shall cancel editing/help/confirmation states, return from task mode to list mode, and never mutate data by itself
  REQ-12 (should): the root README shall document the current TUI launch command and key bindings

scope:
  in: interactive create/rename/delete for task lists; interactive create/rename/delete/complete for tasks; task selection; in-app help; status/error line; README keybinding notes; TestBackend coverage
  out: mouse support; undo; bulk actions; task un-complete; reordering lists/tasks; search/filter; theme configuration; multi-window layout
assumptions:
  - completion remains one-way because the existing CompleteTask use case is one-way
  - duplicate and empty validation reuse existing use case/domain errors
  - input editing can be simple single-line text entry with Backspace, printable characters, Enter, and Esc
  - delete confirmation can be a one-key yes/no prompt
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (adapter unit, red):
    Given the TUI is in list mode / When the user presses ? / Then a help view or overlay lists the list-mode keys and Esc closes it

  AT-2 covers REQ-2 (adapter unit, red):
    Given an empty in-memory repository / When the TUI receives n, the text "work", and Enter in list mode / Then the repository contains a persisted "work" list and the list panel renders it selected

  AT-3 covers REQ-3 (adapter unit, red):
    Given a selected list named "work" with an existing task / When the TUI receives r, the text "office", and Enter in list mode / Then the repository contains the same list renamed to "office" with the task still present

  AT-4 covers REQ-4 (adapter unit, red):
    Given two persisted task lists and the first is selected / When the TUI receives d and confirms yes / Then the first list is removed, the second list remains visible, and the selected index is valid

  AT-5 covers REQ-5 (adapter unit, red):
    Given a selected list with two tasks and task mode is open / When the TUI receives Down then Up / Then the task highlight moves to the second task then back to the first

  AT-6 covers REQ-6 (adapter unit, red):
    Given task mode is open for a selected list / When the TUI receives n, the text "milk", and Enter / Then the repository contains task "milk" in that list and the task panel renders it

  AT-7 covers REQ-7 (adapter unit, red):
    Given task mode is open with task "milk" selected / When the TUI receives r, the text "buy milk", and Enter / Then the repository contains "buy milk" and no longer contains "milk"

  AT-8 covers REQ-8 (adapter unit, red):
    Given task mode is open with task "milk" selected / When the TUI receives d and confirms yes / Then "milk" is removed from the repository and task selection remains valid

  AT-9 covers REQ-9 (adapter unit, red):
    Given task mode is open with the only incomplete task selected / When the TUI receives Space / Then the task is persisted completed and the list panel renders the list as completed

  AT-10 covers REQ-10, REQ-11 (adapter unit, red):
    Given an edit or confirmation is active / When the user submits an invalid duplicate value or presses Esc / Then a visible status/cancel state is rendered and persisted data is unchanged

  AT-11 covers REQ-12 (documentation review):
    Given README.md / When a user reads the TUI section / Then it documents cargo run, ?, q, Esc, list-mode keys, and task-mode keys

  DV-1 covers REQ-1, REQ-2, REQ-3, REQ-4, REQ-5, REQ-6, REQ-7, REQ-8, REQ-9, REQ-10, REQ-11 (manual terminal smoke):
    reason: TestBackend verifies rendering and state transitions, but not crossterm raw terminal ergonomics
    interim check: cargo run in a terminal; use ? help; create/rename/delete a list; create/rename/delete/complete a task; confirm q exits cleanly
    future automated path: a scripted pseudo-terminal e2e test that drives crossterm key events against the binary
    follow-up task: T-8 (manual terminal smoke or pty automation)

design:
  domain: no new domain entities or invariants; reuse TaskList aggregate and Task invariants
  ports: reuse existing TaskListRepository and IdGenerator ports; no new application ports
  adapters:
    driving: extend App in src/adapters/tui_app.rs with interaction state, single-line input, confirmations, help, status messages, task selection, and mutation key handlers
    driven: reuse TomlTaskListRepository in production and InMemoryTaskListRepository/SeqIdGenerator in tests
  usecases:
    task lists: CreateTaskList, ListTaskLists, RenameTaskList, DeleteTaskList
    tasks: ListTasks, AddTask, RenameTask, DeleteTask, CompleteTask
    flow: key event -> App command handler -> existing use case -> refresh AppState from repository -> render status/panels
  feature-local conventions:
    - mode names: Lists, Tasks(list_index), Editing(EditAction), Confirming(ConfirmAction), Help(previous_mode)
    - list-mode keys: ? help, n new list, r rename list, d delete list, Enter open tasks, Up/Down select list, q quit
    - task-mode keys: ? help, n new task, r rename task, d delete task, Space complete task, Up/Down select task, Esc return to list mode, q quit
    - editing keys: printable characters append, Backspace removes, Enter submits, Esc cancels
    - confirmation keys: y confirms, n or Esc cancels
    - status line: one visible line for success/error/cancel feedback; adapters translate use case errors to user-facing text
    - after every successful mutation, App refreshes lists/tasks from the repository rather than mutating rendered state directly
