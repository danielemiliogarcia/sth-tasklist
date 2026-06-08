# spec: panel-focus-colors
intent: render the active panel border and title in LightCyan so the user can see which panel has focus
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): when mode is Lists, the Task Lists panel border/title color shall be Color::LightCyan and the Tasks panel border/title color shall be the terminal default (Color::Reset)
  REQ-2 (must): when mode is Tasks(_), the Tasks panel border/title color shall be Color::LightCyan and the Task Lists panel border/title color shall be the terminal default (Color::Reset)
  REQ-3 (should): the color change shall apply to both the border lines and the title text of the panel block, with no other layout or content changes

scope:
  in: render_lists and render_task_list methods in src/adapters/tui_app.rs; Color import from ratatui::style
  out: domain types, ports, usecases, AppState fields, key bindings, status bar; no new public API
assumptions:
  - ratatui's Block::border_style and Block::title_style (or Block::style) are available; if not, Style on the Block itself applies to borders and title
  - Color::LightCyan is available in ratatui::style::Color (it is in ratatui >= 0.24)
  - the inactive panel uses Color::Reset (terminal default), not a dim/gray value, to keep the change minimal
open questions:
  - none

acceptance:
  AT-1 covers REQ-1, REQ-3 (unit, red):
    Given mode is Lists (initial app state)
    When a frame is rendered via TestBackend (100x28)
    Then at least one cell in the Task Lists border area has fg == Color::LightCyan
    And no cell in the Tasks panel border area has fg == Color::LightCyan

  AT-2 covers REQ-2, REQ-3 (unit, red):
    Given mode is Tasks(_) (after pressing Enter on a list)
    When a frame is rendered via TestBackend (100x28)
    Then at least one cell in the Tasks panel border area has fg == Color::LightCyan
    And no cell in the Task Lists panel border area has fg == Color::LightCyan

  AT-3 covers REQ-1, REQ-2 (unit, red):
    Given mode alternates Lists -> Tasks -> Lists (Enter then Esc)
    When a frame is rendered after each mode change
    Then the highlighted panel switches accordingly (LightCyan tracks the active panel)

design:
  domain: no changes — panel focus is purely a rendering concern
  ports: no changes
  adapters:
    driving: App (tui_app.rs) — render_lists and render_task_list updated:
      - derive active panel from self.state.mode (Lists => lists panel active, Tasks(_) => tasks panel active)
      - helper fn panel_style(active: bool) -> Style returning Style::default().fg(Color::LightCyan) when active, Style::default() when inactive
      - apply panel_style to Block via .border_style(style).title_style(style) (or .style(style) if the former are unavailable)
      - add Color to the existing ratatui::style import line
  usecases: no changes
  feature-local conventions:
    - color choice: Color::LightCyan (ratatui built-in); do not use RGB values to stay theme-neutral
    - panel_style is a private free function or method local to tui_app.rs; not promoted to a port or shared module
    - the inactive style is Color::Reset (not omitted) so future tests can assert explicitly on both states
