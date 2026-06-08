# spec: tui-shell
intent: runnable ratatui TUI shell with real IdGenerator, composition root, list navigation, and task view
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): a UuidIdGenerator shall implement the IdGenerator port, generating unique non-empty string IDs
  REQ-2 (must): the composition root (main.rs) shall wire TomlTaskListRepository and UuidIdGenerator into all task-list and task use cases
  REQ-3 (must): the TUI shall launch without panic, render a visible task-list panel, and exit cleanly when q is pressed
  REQ-4 (must): the TUI shall display every task list from ./.shtask/, showing each list's name and completion status
  REQ-5 (should): the user shall navigate task lists with ↑/↓ arrow keys; the selected list is visually highlighted
  REQ-6 (should): pressing Enter on a selected list shall show its tasks in a second panel with each task's title and completed state

scope:
  in: UuidIdGenerator; composition root; ratatui TUI skeleton; list-panel display + navigation; task-panel view (read-only)
  out: creating / editing / deleting / completing lists or tasks from the TUI (future features); mouse input; config file; colour themes
assumptions:
  - ./.shtask/ may be absent or empty; the TUI shows an empty list without error
  - terminal is large enough to render two side-by-side panels; no responsive resize handling
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (unit):
    Given two calls to UuidIdGenerator.new_id() / When compared / Then the IDs are different and both are non-empty

  DV-1 covers REQ-2 (deferred):
    reason: composition root correctness requires launching the binary, not a unit test
    interim check: cargo build --release succeeds; cargo run exits 0 after pressing q within 5 s
    future automated path: add a CLI flag --headless that runs one tick and exits; assert exit code 0
    follow-up task: T-6 (add --headless integration test)

  DV-2 covers REQ-3 (deferred):
    reason: TUI rendering requires a real terminal; cannot assert frame contents in CI
    interim check: cargo run in a terminal, app starts, UI visible, q exits cleanly
    future automated path: instaframe / ratatui's TestBackend for non-interactive assertions
    follow-up task: T-7 (TestBackend smoke test)

  DV-3 covers REQ-4 (deferred):
    reason: list display correctness requires visual inspection
    interim check: create two lists via the existing usecases, cargo run, confirm both names and ✓ / pending badges are visible
    future automated path: TestBackend renders a frame; assert list names appear in buffer
    follow-up task: T-7 (same as DV-2 follow-up)

  DV-4 covers REQ-5 (deferred):
    reason: keyboard navigation requires a live terminal session to observe
    interim check: run app, press ↑/↓, confirm selection highlight moves correctly
    future automated path: inject key events into TestBackend, assert selected index changes
    follow-up task: T-7

  DV-5 covers REQ-6 (deferred):
    reason: task panel rendering requires visual inspection
    interim check: navigate to a list with tasks, press Enter, confirm tasks panel shows titles and ✓ / ☐ markers
    future automated path: TestBackend frame assertion
    follow-up task: T-7

design:
  domain: no new domain types; UuidIdGenerator is an adapter, not a domain concern
  ports: reuse existing IdGenerator + TaskListRepository ports from task-list-crud; no new ports
  adapters:
    driven: UuidIdGenerator (implements IdGenerator port; uses the uuid crate v4 random IDs)
    driving: App (ratatui driving adapter):
      state model: AppState { lists: Vec<TaskList>, selected_list: usize, mode: Mode }
      Mode enum: Lists | Tasks(usize)  — which panel is focused
      event loop: crossterm raw mode + ratatui Terminal; poll Event::Key each tick
      render: two Paragraph/List widgets — lists panel (left) + tasks panel (right)
      key bindings: q quit; ↑/↓ navigate; Enter enter Tasks mode; Esc return to Lists mode
  usecases: uses existing ListTaskLists + ListTasks from task-list-crud + task-crud; no new use cases
  feature-local conventions:
    - new driven adapter: UuidIdGenerator (uuid crate)
    - new driving adapter: App struct + main event loop (src/adapters/tui_app.rs)
    - composition root lives in src/main.rs; it is the only place TomlTaskListRepository + UuidIdGenerator are constructed
    - AppState is not domain — it lives in the adapter layer; it is rebuilt from the repo on each action that mutates state
    - follow-up tasks T-6/T-7 are intentionally deferred; do not block the feature on them
