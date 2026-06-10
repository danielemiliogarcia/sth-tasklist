# spec: auto-display-list
intent: auto-preview tasks in the right pane whenever focus moves in the tasklist pane, without requiring an extra keypress
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): when focus is on the tasklist pane and the user moves up/down, the right pane immediately updates to show tasks for the highlighted tasklist (no extra keypress needed)
  REQ-2 (must): entering a tasklist (to add/remove/mark-done a task) still requires an explicit key (right arrow or tab), unchanged
  REQ-3 (should): if the highlighted tasklist is empty, the right pane shows an empty state (not stale content from the previous selection)

scope:
  in: select_next/select_previous in tui_app.rs; AppState; render_task_list; refresh()
  out: mode transitions; CRUD operations; help text changes; mouse input
assumptions:
  - preview is read-only; no selection cursor or task interaction while in Lists mode
  - mode stays Lists during preview — entering still requires Right/Enter/Tab
  - preview also initialised on app start and after refresh when in Lists mode
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (adapter unit, red):
    Given app in Lists mode with two lists / When Down arrow pressed / Then right pane immediately shows tasks for newly highlighted list (no Enter needed)

  AT-2 covers REQ-2 (adapter unit):
    Given app in Lists mode / When Right arrow or Tab pressed / Then mode becomes Tasks and right pane enters interactive task mode

  AT-3 covers REQ-3 (adapter unit, red):
    Given app in Lists mode and highlighted list has no tasks / When user navigates to it / Then right pane shows empty state (not stale content from prior list)

  DV-1 covers REQ-1, REQ-2, REQ-3 (manual terminal smoke):
    reason: TestBackend verifies state transitions but not live terminal feel
    interim check: cargo run; navigate lists with Up/Down and confirm right pane updates per list; press Right/Tab and confirm task interaction still works; navigate to empty list and confirm no stale content
    future automated path: AT-1..AT-3 cover the state transitions; smoke is one-off
    follow-up task: T-5

design:
  domain: no new entities or invariants
  ports: no new ports; existing ListTasks use case reused
  adapters:
    driving: extend select_next/select_previous in tui_app.rs — when Mode::Lists, call new load_preview() helper after updating selected_list; also call load_preview() in refresh() when mode is Lists
    state: add preview_tasks: Vec<Task> to AppState; cleared in close_tasks() and clamp_selection()
    render: update render_task_list — when Mode::Lists, render preview_tasks (empty state if vec is empty) instead of "No task list open"; no selection cursor or active highlight in preview mode
  usecases: no new use cases; load_preview() delegates to ListTasks
  feature-local conventions:
    - preview_tasks is display-only; no selection index; no CRUD while in Lists mode
    - mode stays Lists during preview — entering requires Right/Enter/Tab as before
