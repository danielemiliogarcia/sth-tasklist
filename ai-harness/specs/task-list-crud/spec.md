# spec: task-list-crud
intent: create, list, rename, and delete task lists, persisted as TOML files
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): the system shall create a task list from a name, rejecting an empty or whitespace-only name (EmptyName)
  REQ-2 (must): the system shall reject creating a task list whose name already exists (DuplicateName, case-sensitive)
  REQ-3 (must): the system shall return all existing task lists
  REQ-4 (must): the system shall rename an existing task list, applying the same non-empty + unique rules (NotFound if absent)
  REQ-5 (must): the system shall delete an existing task list (NotFound if absent)
  REQ-6 (must): task lists shall persist across runs as TOML, one file per list, under ./.shtask/

scope:
  in: CRUD of task lists; non-empty + unique name rule; TOML persistence under ./.shtask/
  out: tasks inside a list; task/list completion; TUI rendering (later features)
assumptions:
  - name comparison is case-sensitive and exact (no trimming beyond the empty/whitespace check)
  - each list has a generated stable id; the TOML filename derives from the id
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (unit, red):
    Given a blank name / When create / Then raise EmptyName and nothing is stored
  AT-2 covers REQ-1 (unit):
    Given name "work" / When create / Then a list with that name and a new id exists
  AT-3 covers REQ-2 (unit, red):
    Given a list "work" exists / When create "work" / Then raise DuplicateName
  AT-4 covers REQ-3 (unit):
    Given lists "work" and "home" exist / When list all / Then both are returned
  AT-5 covers REQ-4 (unit):
    Given list "work" exists / When rename to "office" / Then its name is "office"
  AT-6 covers REQ-4 (unit, red):
    Given lists "work" and "home" exist / When rename "home" to "work" / Then raise DuplicateName
  AT-7 covers REQ-5 (unit):
    Given list "work" exists / When delete it / Then it is no longer returned by list all
  AT-8 covers REQ-6 (integration):
    Given a list created via the TOML repository / When a fresh repository loads ./.shtask/ / Then the list is present

design:
  domain: TaskList (id, name; invariant: name is non-empty/non-whitespace). Uniqueness is a collection rule enforced by usecases, not a single-entity invariant.
  ports: TaskListRepository (save, all, by_id, find_by_name, delete)
  adapters:
    driven: TomlTaskListRepository — reads/writes one ./.shtask/<id>.toml per list; InMemoryTaskListRepository — test double for usecase tests
    driving: (deferred — a later tui feature drives these usecases)
  usecases:
    CreateTaskList: validate name non-empty -> find_by_name to enforce unique -> build TaskList -> save -> return id
    ListTaskLists: repo.all
    RenameTaskList: by_id (NotFound) -> validate new name non-empty -> find_by_name to enforce unique -> update -> save
    DeleteTaskList: by_id (NotFound) -> repo.delete
  feature-local conventions:
    - new port TaskListRepository
    - errors: EmptyName, DuplicateName, NotFound (feature-local error enum)
    - id: generated short stable id; ./.shtask/<id>.toml is the on-disk form
    - storage dir ./.shtask/ created on first write
