# state: colour-theme
feature: colour-theme
branch: feat/colour-theme
phase: done
overall: done
updated: 2026-06-08
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: all 8 AT tests green; domain + port + adapter + tui wiring + main.rs done; 66 total tests pass
next: none — spec complete
blocker: none

read budget:
  now: this file
  on demand: src/domain/colour_theme.rs · src/adapters/toml_theme.rs · src/adapters/tui_app.rs · src/main.rs
  skip: other features · whole context/

tasks:
  T-1 done  ColourTheme domain value object + AT-1            (REQ-1, AT-1)
  T-2 done  ThemeRepository port declaration                  (REQ-2)
  T-3 done  TomlThemeRepository driven adapter + AT-2/3/4    (REQ-2, REQ-3, AT-2, AT-3, AT-4)
  T-4 done  Wire ColourTheme into App constructor (tui_app)  (REQ-4, AT-5, AT-6, AT-7, AT-8)
  T-5 done  Composition root wiring in main.rs               (REQ-5, DV-1)
  T-6 todo  DV-1 follow-up: cargo build + manual smoke test  (REQ-5, DV-1)
  T-7 todo  DV-2 follow-up: named-color manual check         (REQ-7, DV-2)

watch-outs:
  - NamedColor enum lives in domain; named_to_color() conversion in tui_app.rs (adapter boundary)
  - selected_item_reverse flag: true = REVERSED modifier, false = explicit fg/bg
  - colours.toml path is ./colours.toml (cwd-relative); documented in main.rs comment
  - T-6/T-7 are manual smoke checks; they do not block spec completion
