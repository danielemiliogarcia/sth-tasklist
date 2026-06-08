# spec: modal-input
intent: guard quit and navigation keys so they are inactive while an editing or confirming interaction is active
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): when interaction is Editing, every printable character (including 'q', 'n', 'r', 'd', '?') shall be appended to the input buffer and shall not trigger any navigation or quit side-effect
  REQ-2 (must): when interaction is Confirming, printable characters other than 'y'/'Y'/'n'/'N' (including 'q') shall be silently ignored and shall not quit or navigate
  REQ-3 (must): quit ('q') shall only terminate the application when interaction is None or Help
  REQ-4 (should): navigation keys (Up, Down, Enter, Esc) shall only affect mode or selection when interaction is None; Esc while Editing or Confirming shall cancel as already defined by tui-crud-interactions/REQ-11

scope:
  in: fix key-dispatch order in handle_key in src/adapters/tui_app.rs; add regression tests with TestBackend covering the bug scenario
  out: new interaction types; changes to EditAction or ConfirmAction variants; any domain or use-case layer changes; mouse input
assumptions:
  - handle_edit_key and handle_confirm_key already implement the correct character routing; only the early-return 'q' guard in handle_key is wrong
  - Esc cancellation behavior in Editing/Confirming is correct and must not be altered
open questions:
  - none

acceptance:
  AT-1 covers REQ-1, REQ-3 (adapter unit, red):
    Given the app is in Editing interaction (e.g. CreateList triggered by 'n') /
    When the key 'q' is sent via handle_key /
    Then handle_key returns Ok(false) and the input buffer contains "q" and the app has not quit

  AT-2 covers REQ-1 (adapter unit, red):
    Given the app is in Editing interaction /
    When the sequence of chars "termotanque" is sent via handle_key /
    Then handle_key returns Ok(false) for each key and the input buffer equals "termotanque"

  AT-3 covers REQ-2, REQ-3 (adapter unit, red):
    Given the app is in Confirming interaction (e.g. DeleteList triggered by 'd') /
    When the key 'q' is sent via handle_key /
    Then handle_key returns Ok(false) and the interaction remains Confirming and the list is not deleted

  AT-4 covers REQ-3 (adapter unit):
    Given interaction is None /
    When the key 'q' is sent via handle_key /
    Then handle_key returns Ok(true) (signals quit)

  AT-5 covers REQ-4 (adapter unit):
    Given the app is in Editing interaction /
    When keys Up, Down, and Enter are sent via handle_key /
    Then handle_key returns Ok(false) for each, the selected_list index is unchanged, and mode is unchanged

design:
  domain: none — this is a pure adapter-layer fix; no domain entities or invariants are touched
  ports: none — reuse existing TaskListRepository and IdGenerator ports; no new ports
  adapters:
    driving: fix App::handle_key in src/adapters/tui_app.rs
      current (broken) order:
        1. if q -> quit (fires regardless of interaction)
        2. match interaction { None | Help | Editing | Confirming }
      corrected order:
        1. match interaction { None | Help -> check q here; Editing -> delegate to handle_edit_key; Confirming -> delegate to handle_confirm_key }
      the 'q' early-return must move inside the None/Help branch only;
      handle_edit_key and handle_confirm_key already handle their own key sets correctly
  usecases: none — no use case changes
  feature-local conventions:
    - "modal key guard": the principle that quit/navigation keys are gated by the current interaction, not checked globally
    - fix is a one-site change in handle_key; no new types or enums needed
