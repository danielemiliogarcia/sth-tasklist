# spec: uncomplete-task
intent: toggle task completion with Space — completing an incomplete task and un-completing a completed one

requirements:
  REQ-1 (must): the domain shall support marking a completed task as incomplete (reversing completion)
  REQ-2 (must): the TaskList aggregate shall expose an uncomplete_task method that resets a task to incomplete, reporting NotFound for unknown titles
  REQ-3 (must): an UncompleteTask use case shall load the list, call uncomplete_task, and persist the result via the repository port
  REQ-4 (must): pressing Space on a completed task shall call UncompleteTask and leave the task incomplete
  REQ-5 (must): pressing Space on an incomplete task shall call CompleteTask and leave the task complete (existing behaviour, unchanged)
  REQ-6 (must): the status bar and the help screen shall read "Space: toggle complete" (not "Space: complete")

acceptance:
  AT-1 covers REQ-1, REQ-2 (unit, red):
    Given a Task whose completed field is true
    When mark_uncompleted() is called on the Task
    Then is_completed() returns false

  AT-2 covers REQ-2 (unit, red):
    Given a TaskList with a completed task "milk"
    When uncomplete_task("milk") is called
    Then tasks()[0].is_completed() is false and Ok(()) is returned

  AT-3 covers REQ-2 (unit, red):
    Given a TaskList with no task titled "ghost"
    When uncomplete_task("ghost") is called
    Then Err(TaskError::NotFound) is returned and no task is mutated

  AT-4 covers REQ-3 (unit, red):
    Given a repository containing a list with a completed task "milk"
    When UncompleteTask::execute(list_id, "milk") is called
    Then the repository's saved copy of "milk" has is_completed() == false

  AT-5 covers REQ-4, REQ-5 (integration, red):
    Given the TUI with a list containing completed task "milk" selected
    When the Space key is pressed
    Then the task is incomplete afterwards; pressing Space again makes it complete

  AT-6 covers REQ-6 (unit):
    Given the TUI app is rendered in task-browsing mode with a task selected
    When the status bar text and help-screen text are inspected
    Then both contain "toggle complete" and neither contains "Space: complete" alone

design:
  domain:
    Task — add pub(crate) mark_uncompleted(&mut self) that sets completed = false; idempotent
    TaskList — add pub uncomplete_task(&mut self, title: &str) -> Result<(), TaskError> mirroring complete_task

  ports:
    TaskListRepository (save, by_id) — no change; existing port already sufficient

  adapters:
    driven: TomlTaskListRepository — no change needed (persists full Task state via serde)
    driven (test): InMemoryTaskListRepository — no change needed
    driving: TuiApp —
      rename complete_selected_task -> toggle_selected_task (or keep name, change body);
      inspect task.is_completed() to branch between CompleteTask and UncompleteTask;
      update status-bar string and help-screen ListItem text

  usecases:
    UncompleteTask<R: TaskListRepository>:
      load list via repo.by_id -> call list.uncomplete_task(title) -> repo.save(list)
      mirrors CompleteTask exactly

  feature-local conventions:
    mark_uncompleted is pub(crate) on Task, matching the existing visibility of mark_completed
