# state: green-completed-tasks
feature: green-completed-tasks
branch: codex-continue
phase: done
overall: done
updated: 2026-06-08
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: all 3 AT tests green; domain field + TOML wiring + task_items() style applied; 76 tests pass
next: none — spec complete
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md design section
  on demand: ../../context/architecture.md · src/domain/colour_theme.rs · src/adapters/tui_app.rs · src/adapters/toml_theme.rs
  skip: other features · whole context/

tasks:
  T-1 todo  Add completed_task_fg: NamedColor (default Green) to ColourTheme domain VO                (REQ-2, AT-1)
  T-2 todo  Add completed_task_fg to ColourThemeDto + parse_color wiring in TomlThemeRepository       (REQ-3, AT-3)
  T-3 todo  Apply completed_task_fg style to completed items in task_items() in TuiApp                (REQ-1, AT-2)
  T-4 todo  Add optional completed_task_fg key to colours.toml                                        (REQ-3, AT-3)
  T-5 todo  Manual DV-1 visual check; add TestBackend render test when infrastructure allows          (REQ-1, DV-1)

watch-outs:
  - task_items() currently uses selection style for all items; the completed-task fg must be applied after (or instead of) any selection-state branching so it wins regardless of focus
  - named_to_color() is a free function in tui_app.rs; it already handles all NamedColor variants — no changes expected there unless a new variant is added
  - colours.toml field name must be exactly completed_task_fg to stay consistent with existing keys
