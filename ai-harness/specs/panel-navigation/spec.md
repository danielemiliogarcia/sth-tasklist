# spec: panel-navigation
intent: add Right/Left arrow keys as aliases for Enter/Esc to switch between the lists and tasks panels
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): pressing Right while in Lists mode and at least one list exists shall open the tasks panel for the selected list (identical effect to Enter)
  REQ-2 (must): pressing Left while in Tasks mode shall return to Lists mode (identical effect to Esc)
  REQ-3 (must): existing Enter and Esc bindings shall remain fully operational and unchanged

scope:
  in: Right/Left arrow key handling in handle_normal_key; help text update for both modes
  out: mouse input; new domain types; new ports; new use cases; any change to rendering logic beyond help text
assumptions:
  - Right does nothing when lists is empty (mirrors Enter behavior which is guarded by state.lists.get check)
  - Left does nothing when already in Lists mode (mirrors Esc guard)
  - Right/Left are ignored during Editing, Confirming, and Help interactions (handled by existing interaction dispatch)
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (adapter unit, red):
    Given a seeded app in Lists mode with "work" selected / When Right arrow is pressed / Then mode becomes Tasks(0) and the tasks panel renders the tasks for "work"

  AT-2 covers REQ-2 (adapter unit, red):
    Given a seeded app after entering Tasks mode / When Left arrow is pressed / Then mode becomes Lists and the right panel renders "No task list open"

  AT-3 covers REQ-1, REQ-3 (adapter unit, red):
    Given a seeded app in Lists mode / When Enter is pressed then app is reset to Lists mode / When Right is pressed / Then both produce mode Tasks(0) — Right and Enter are equivalent

  AT-4 covers REQ-2, REQ-3 (adapter unit, red):
    Given a seeded app in Tasks mode / When Esc is pressed then app re-enters Tasks mode / When Left is pressed / Then both return to Lists mode — Left and Esc are equivalent

  AT-5 covers REQ-1 (adapter unit, red):
    Given an empty app with no lists / When Right arrow is pressed / Then mode remains Lists and no panic occurs

  DV-1 covers REQ-1, REQ-2, REQ-3 (manual terminal smoke):
    reason: TestBackend verifies state transitions but not crossterm key event feel in a live terminal
    interim check: cargo run; navigate to a list with Right; confirm tasks panel opens; press Left; confirm lists panel regains focus; confirm Enter/Esc still work independently
    future automated path: existing TestBackend harness already covers this via AT-1..AT-5
    follow-up task: none (AT tests are sufficient; smoke check is one-off manual verification)

design:
  domain: no new domain entities or invariants
  ports: no new ports; no changes to existing ports
  adapters:
    driving: extend handle_normal_key in src/adapters/tui_app.rs with two new match arms:
      KeyCode::Right -> if mode == Lists, call self.open_tasks() (same as Enter arm)
      KeyCode::Left  -> if mode == Tasks(_), reset mode to Lists and clear tasks (same as Esc arm)
    also update render_help list-mode and task-mode item lists to include Right/Left bindings
  usecases: no new use cases; Right delegates to existing open_tasks(); Left reuses inline reset already in the Esc arm
  feature-local conventions:
    - Right and Left are aliases only — they share the same implementation path as Enter/Esc respectively; do not duplicate logic, prefer calling open_tasks() directly
    - help text lines to add: "Right open tasks" (list mode), "Left return to lists" (task mode)
    - no new enums, structs, or traits are introduced by this feature
