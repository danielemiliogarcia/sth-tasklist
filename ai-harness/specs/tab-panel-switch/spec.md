# spec: tab-panel-switch
intent: add Tab as a panel-cycling alias so the user can switch between lists and tasks panels with a single key
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): pressing Tab while in Lists mode shall switch to Tasks mode for the selected list (identical effect to Enter/Right)
  REQ-2 (must): pressing Tab while in Tasks mode shall return to Lists mode (identical effect to Esc/Left)
  REQ-3 (should): the help screen shall be updated to mention Tab as a panel-switching key in both modes
  REQ-4 (could): pressing Shift+Tab shall cycle panels in the reverse direction (Tasks → Lists and Lists → Tasks)

scope:
  in: Tab (and optionally Shift+Tab) key handling in handle_normal_key; help text update for both modes
  out: mouse input; new domain types; new ports; new use cases; any change to rendering logic beyond help text
assumptions:
  - Tab in Lists mode with no lists is a no-op (mirrors Enter/Right guard inside open_tasks())
  - Tab in Lists mode has no effect when lists is empty (same guard as Enter)
  - Tab/Shift+Tab are ignored during Editing, Confirming, and Help interactions (existing dispatch handles those modes)
  - Existing Enter, Esc, Right, Left bindings are entirely unaffected
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (adapter unit, red):
    Given a seeded app in Lists mode with at least one list selected / When Tab is pressed / Then mode becomes Tasks(0) and the tasks panel renders tasks for the selected list

  AT-2 covers REQ-2 (adapter unit, red):
    Given a seeded app in Tasks mode / When Tab is pressed / Then mode becomes Lists and the right panel renders "No task list open"

  AT-3 covers REQ-1, REQ-2 (adapter unit, red):
    Given a seeded app in Lists mode / When Tab is pressed then Tab is pressed again / Then mode returns to Lists — Tab cycles forward then backward across two presses

  AT-4 covers REQ-1 (adapter unit, red):
    Given an app with no lists in Lists mode / When Tab is pressed / Then mode remains Lists and no panic occurs

  AT-5 covers REQ-3 (adapter unit):
    Given a seeded app in Lists mode / When Tab is pressed / Then the rendered help text contains "Tab" for the panel-switch binding

  AT-6 covers REQ-4 (adapter unit, red):
    Given a seeded app in Tasks mode / When Shift+Tab is pressed / Then mode becomes Lists

  AT-7 covers REQ-4 (adapter unit, red):
    Given a seeded app in Lists mode with at least one list selected / When Shift+Tab is pressed / Then mode becomes Tasks(0)

  DV-1 covers REQ-1, REQ-2, REQ-3 (manual terminal smoke):
    reason: TestBackend verifies state transitions but not crossterm key event feel in a live terminal
    interim check: cargo run; press Tab to open tasks panel; press Tab again to return to lists; confirm help text shows Tab; confirm Enter/Esc/Right/Left still work independently
    future automated path: existing TestBackend harness already covers this via AT-1..AT-5
    follow-up task: none (AT tests are sufficient; smoke check is one-off manual verification)

design:
  domain: no new domain entities or invariants
  ports: no new ports; no changes to existing ports
  adapters:
    driving: extend handle_normal_key in src/adapters/tui_app.rs with new match arms:
      KeyCode::Tab -> if mode == Lists, call self.open_tasks(); if mode == Tasks(_), reset mode to Lists and clear tasks
      KeyCode::BackTab (Shift+Tab) -> same logic as Tab but direction reversed (optional REQ-4)
    also update render_help list-mode and task-mode help item lists to include Tab binding description
  usecases: no new use cases; Tab delegates to existing open_tasks() for forward switch; backward switch reuses inline reset already in the Esc/Left arm
  feature-local conventions:
    - Tab is an alias only — it shares the same implementation path as Enter/Right (forward) and Esc/Left (backward); do not duplicate logic, call open_tasks() directly
    - KeyCode::BackTab is how crossterm/ratatui represents Shift+Tab; no special setup needed
    - help text lines to add: "Tab switch panel" (list mode and task mode, or a single combined line)
    - no new enums, structs, or traits are introduced by this feature
