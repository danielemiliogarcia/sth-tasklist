# spec: hotkey-bar
intent: add a permanent one-line hotkey bar at the bottom of the TUI that always shows context-sensitive key bindings

requirements:
  REQ-1 (must): the TUI shall display a dedicated hotkey bar as the lowest row of the terminal, always visible regardless of the current status message
  REQ-2 (must): when mode is Lists and interaction is None, the hotkey bar shall show: n: new  r: rename  d: delete  Enter/Tab: open tasks  q: quit
  REQ-3 (must): when mode is Tasks and interaction is None, the hotkey bar shall show: n: new  r: rename  d: delete  Space: toggle  Esc/Tab: lists  q: quit
  REQ-4 (must): when interaction is Editing, the hotkey bar shall show: Enter: submit  Esc: cancel
  REQ-5 (must): when interaction is Confirming, the hotkey bar shall show: y: confirm  n/Esc: cancel
  REQ-6 (must): the hotkey bar shall have height 1 and no border, leaving the existing status bar (height 3, above it) unchanged
  REQ-7 (should): the hotkey bar text shall be styled to be visually distinct but low-prominence (e.g. dark/muted foreground) so it does not compete with the status message

acceptance:
  AT-1 covers REQ-1, REQ-6 (unit):
    Given the vertical layout is constructed / When render() runs / Then the layout contains three slices: Min(1), Length(3), Length(1); the third slice is passed to render_hotkeys
  AT-2 covers REQ-2 (unit):
    Given mode=Lists and interaction=None / When render_hotkeys is called / Then the returned Paragraph text contains "n: new" and "Enter/Tab: open tasks" and "q: quit"
  AT-3 covers REQ-3 (unit):
    Given mode=Tasks(_) and interaction=None / When render_hotkeys is called / Then the returned Paragraph text contains "Space: toggle" and "Esc/Tab: lists" and "q: quit"
  AT-4 covers REQ-4 (unit):
    Given interaction=Editing(_) / When render_hotkeys is called / Then the returned Paragraph text contains "Enter: submit" and "Esc: cancel" and does not contain "q: quit"
  AT-5 covers REQ-5 (unit):
    Given interaction=Confirming(_) / When render_hotkeys is called / Then the returned Paragraph text contains "y: confirm" and "n/Esc: cancel" and does not contain "q: quit"
  DV-1 covers REQ-6, REQ-7 (manual, pre-review):
    Reason: ratatui rendering cannot be asserted at pixel level in unit tests without a test backend; visual style requires human judgement
    Interim check: run `cargo run`, resize terminal to narrow width, verify hotkey bar is always present below status bar and styled with muted color
    Future path: integration test using ratatui TestBackend to snapshot rendered buffer
    Follow-up: add T-5 (todo) in state.md to write TestBackend snapshot test post-merge

design:
  domain: none — purely a rendering concern; no domain entities or invariants change
  ports: none — no new port interfaces required
  adapters:
    driving: TuiApp (tui_app.rs) — the only file modified
      - render(): add third Constraint::Length(1) slice to the vertical Layout, bind to hotkey_area; call self.render_hotkeys(frame, hotkey_area)
      - render_hotkeys(&self, frame: &mut Frame, area: Rect): match on (self.state.mode, &self.state.interaction) to select the appropriate hotkey string; render a borderless Paragraph with muted style
  usecases: none
  feature-local conventions:
    - render_hotkeys is a private method on App<R,I>, parallel to render_status
    - hotkey strings are inline string literals inside render_hotkeys match arms (no separate config)
    - Help interaction falls back to Editing display (Enter: submit  Esc: cancel) or can reuse None/Lists display — implementer's choice, not observable from outside
