# spec: colour-theme
intent: load an optional colours.toml at startup and apply user-defined colors to panel borders and selected-row highlights in the TUI
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): a ColourTheme domain value object shall hold named color values for active_panel_border, inactive_panel_border, selected_item_fg, selected_item_bg, and normal_item_fg, with a Default impl that matches the current hardcoded values (LightCyan border, Reset inactive border, REVERSED for selected rows encoded as explicit fg/bg)
  REQ-2 (must): a ThemeRepository port shall expose a single load() -> ColourTheme method; implementations must not panic; a missing file returns the default theme
  REQ-3 (must): a TomlThemeRepository driven adapter shall implement ThemeRepository by reading ./colours.toml; if the file is absent it returns the default theme; if the file is present but malformed it emits a warning to stderr and returns the default theme
  REQ-4 (must): the TUI adapter (tui_app.rs) shall accept a ColourTheme at construction time and use its values in panel_style() (border/title) and in the highlight_style applied to List widgets, replacing all hardcoded Color and Modifier::REVERSED references in those two call sites
  REQ-5 (must): the composition root (main.rs) shall construct a TomlThemeRepository, call load(), and pass the resulting ColourTheme into the App constructor before entering the event loop
  REQ-6 (should): when colours.toml is absent the app behavior shall be visually identical to the pre-feature state (LightCyan active border, Reset inactive border, REVERSED-equivalent selected row)
  REQ-7 (could): colours.toml shall support all named ratatui Color variants (Black, Red, Green, Yellow, Blue, Magenta, Cyan, White and their Light* counterparts, plus Reset) as unquoted strings; unrecognized strings fall back to the field default

scope:
  in: ColourTheme value object (domain); ThemeRepository port; TomlThemeRepository driven adapter; tui_app.rs updated to use theme; main.rs wiring; colours.toml format
  out: per-list or per-task custom colors; mouse interaction; dynamic reload at runtime; GUI color pickers; any font or layout changes
assumptions:
  - colours.toml lives at the current working directory (./colours.toml), the same convention as ./.shtask/
  - ratatui Color enum variants are sufficient; RGB tuple colors are out of scope for now
  - the Default ColourTheme encodes the current REVERSED selected-row behavior as fg=Reset/bg=Reset with a separate use_reverse flag, OR as explicit fg=Black/bg=LightCyan — pick the simplest representation that lets the TUI reconstruct the correct Style; this decision is deferred to implementation (T-1)
  - stderr warning on malformed file is acceptable UX; no in-TUI error display needed
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (unit, red):
    Given ColourTheme::default() is constructed
    When its fields are inspected
    Then active_panel_border == Color::LightCyan, inactive_panel_border == Color::Reset, and selected_item fields match the pre-feature highlight appearance

  AT-2 covers REQ-2, REQ-3 (unit, red):
    Given a TomlThemeRepository pointed at a directory with no colours.toml
    When load() is called
    Then it returns ColourTheme::default() without error

  AT-3 covers REQ-3 (unit, red):
    Given a TomlThemeRepository pointed at a colours.toml with valid content (e.g. active_panel_border = "Green")
    When load() is called
    Then it returns a ColourTheme with active_panel_border == Color::Green and all other fields at their defaults

  AT-4 covers REQ-3 (unit, red):
    Given a TomlThemeRepository pointed at a colours.toml containing invalid TOML
    When load() is called
    Then it returns ColourTheme::default() (no panic) and a warning has been written to stderr

  AT-5 covers REQ-4, REQ-6 (unit, red):
    Given an App constructed with ColourTheme::default()
    When a frame is rendered via TestBackend (100x28) in Lists mode
    Then at least one cell in the Task Lists border area has fg == Color::LightCyan
    And no cell in the Tasks panel border area has fg == Color::LightCyan
    (same assertion as panel-focus-colors/AT-1; this test replaces it under the new wiring)

  AT-6 covers REQ-4 (unit, red):
    Given an App constructed with a ColourTheme where active_panel_border = Color::Green
    When a frame is rendered via TestBackend in Lists mode
    Then at least one cell in the Task Lists border area has fg == Color::Green

  AT-7 covers REQ-4, REQ-6 (unit, red):
    Given an App constructed with ColourTheme::default()
    When a frame is rendered via TestBackend and a selected row exists in the task-lists panel
    Then the selected row cell fg/bg matches the default selected_item appearance (REVERSED or explicit colors per the chosen encoding)

  AT-8 covers REQ-4 (unit, red):
    Given an App constructed with a ColourTheme where selected_item_bg = Color::Yellow and selected_item_fg = Color::Black
    When a frame is rendered via TestBackend with a selected row in the task-lists panel
    Then the selected row cell has fg == Color::Black and bg == Color::Yellow

  DV-1 covers REQ-5 (deferred):
    reason: composition root correctness requires launching the binary
    interim check: cargo build --release succeeds; cargo run with no colours.toml exits cleanly on q; with a valid colours.toml the active border color changes visually
    future automated path: --headless flag (tui-shell/T-6) renders one tick and exits 0
    follow-up task: T-6

  DV-2 covers REQ-7 (deferred):
    reason: full named-color round-trip coverage is low value to test exhaustively at unit level
    interim check: place colours.toml with active_panel_border = "Magenta"; cargo run; verify Magenta border
    future automated path: table-driven unit test over all named variants
    follow-up task: T-7

design:
  domain:
    ColourTheme — value object; fields: active_panel_border: Color, inactive_panel_border: Color, selected_item_fg: Color, selected_item_bg: Color, normal_item_fg: Color; derives Default (matching current hardcoded values); pure, no IO; Color is re-exported from ratatui::style or a thin newtype if serde coupling is undesirable
    note: if serde on ratatui::Color requires a wrapper, introduce a ThemeColor enum in domain with From<ThemeColor> for ratatui::style::Color in the adapter layer

  ports:
    ThemeRepository — one method: fn load(&self) -> ColourTheme; infallible by contract (errors are absorbed inside the impl with a stderr warning)

  adapters:
    driven:
      TomlThemeRepository — implements ThemeRepository; field: path: PathBuf (defaults to ./colours.toml); reads file, deserializes via a ColourThemeDto (serde struct with String fields), maps strings to Color variants, fills missing fields from ColourTheme::default(); on any error emits eprintln! warning and returns default
    driving:
      App (tui_app.rs) — gains a theme: ColourTheme field; constructor fn new(..., theme: ColourTheme) -> Self; panel_style(active: bool) replaced by self.panel_style(active: bool) using theme.active_panel_border / inactive_panel_border; highlight_style built from theme.selected_item_fg and theme.selected_item_bg instead of Modifier::REVERSED

  usecases: no changes

  feature-local conventions:
    - ColourTheme lives in src/domain/colour_theme.rs; ThemeRepository in src/application/ports.rs (alongside existing ports)
    - TomlThemeRepository lives in src/adapters/toml_theme.rs, following the same pattern as src/adapters/toml_repo.rs
    - colours.toml format: flat TOML table; all keys optional; string values are ratatui Color variant names (case-insensitive match recommended)
    - the panel_style free function in tui_app.rs becomes a method or is replaced by inline Style construction using theme fields; do not promote theme lookup to a port call inside render — pass theme at construction only
    - Color is used directly from ratatui::style in the domain only if it has no IO; if that causes serde coupling, use a local NamedColor enum in domain and convert at the adapter boundary
