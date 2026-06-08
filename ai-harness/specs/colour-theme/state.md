# state: colour-theme
feature: colour-theme
branch: feat/colour-theme
phase: tasks
overall: todo
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: spec and state files written; no code started
next: create src/domain/colour_theme.rs with ColourTheme struct, Default impl, and unit test AT-1 (T-1, red first)
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md design section
  on demand: ../../context/architecture.md · src/adapters/toml_repo.rs (pattern reference) · src/adapters/tui_app.rs lines 638-687 and 772-778 (call sites)
  skip: other features · whole context/ · src/domain/* (except colour_theme.rs being created)

tasks:
  T-1 todo  ColourTheme domain value object + AT-1            (REQ-1, AT-1)
  T-2 todo  ThemeRepository port declaration                  (REQ-2)
  T-3 todo  TomlThemeRepository driven adapter + AT-2/3/4    (REQ-2, REQ-3, AT-2, AT-3, AT-4)
  T-4 todo  Wire ColourTheme into App constructor (tui_app)  (REQ-4, AT-5, AT-6, AT-7, AT-8)
  T-5 todo  Composition root wiring in main.rs               (REQ-5, DV-1)
  T-6 todo  DV-1 follow-up: cargo build + manual smoke test  (REQ-5, DV-1)
  T-7 todo  DV-2 follow-up: named-color manual check         (REQ-7, DV-2)

watch-outs:
  - ColourTheme must live in domain (pure, no IO); if ratatui::style::Color pulls in serde, introduce a NamedColor enum in domain and convert at the adapter boundary — decide in T-1 before writing T-2/T-3
  - The current highlight_style uses Modifier::REVERSED (terminal inversion), not explicit fg/bg; the default ColourTheme must reproduce this visually — simplest encoding is a use_reverse: bool flag, or map to explicit fg=Black/bg=LightCyan; commit to one in T-1
  - panel_style is currently a free function; it will become a method or inline call using self.theme — do not try to keep the free-function signature
  - colours.toml path is ./colours.toml (relative to cwd at launch); document this in a comment in main.rs or in the repo README if one exists
  - AT-5 replaces panel-focus-colors/AT-1 semantically; if those tests already exist, update them rather than duplicating
