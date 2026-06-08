# state: panel-focus-colors
feature: panel-focus-colors
branch: panel-focus-colors
phase: done
overall: done
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: all 3 AT tests green; panel_style helper added; border_style+title_style applied in render_lists and render_task_list; all 58 tests pass
next: none — spec complete
blocker: none

read budget:
  now: this file
  on demand: src/adapters/tui_app.rs
  skip: other features · whole context/

tasks:
  T-1 done write AT-1 red test: LightCyan on lists border when mode=Lists (REQ-1, AT-1)
  T-2 done write AT-2 red test: LightCyan on tasks border when mode=Tasks(_) (REQ-2, AT-2)
  T-3 done write AT-3 red test: color switches on mode toggle (REQ-1, REQ-2, AT-3)
  T-4 done add Color import and panel_style helper in tui_app.rs (REQ-1, REQ-2, REQ-3)
  T-5 done apply panel_style to Block in render_lists and render_task_list; AT-1/AT-2/AT-3 green (REQ-1, REQ-2, REQ-3)

watch-outs:
  - panel_style is a free function (not method); takes active: bool; returns Style with LightCyan or Reset
  - border_style and title_style both applied; title_style colors only the title chars, border_style colors the box lines
  - render_help/render_editor/render_confirmation not modified (out of scope per spec)
  - area_has_cyan test helper + render_terminal helper added to test module
