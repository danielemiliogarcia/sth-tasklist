# state: panel-focus-colors
feature: panel-focus-colors
branch: panel-focus-colors
phase: tasks
overall: todo
updated: 2026-06-07
# legal values (phases, task states), task-line format, how to fill: see ../global-state-info.md

last step: spec and state files written; no code changed yet
next: write AT-1 (TestBackend unit test asserting LightCyan fg on the lists panel border when mode is Lists) as a red test in src/adapters/tui_app.rs, then implement panel_style helper and apply it in render_lists/render_task_list
blocker: none

read budget:
  now: ../../START-HERE.md (if unread) · this file · spec.md design section
  on demand: ../../context/architecture.md
  skip: other features · whole context/

tasks:
  T-1 todo write AT-1 red test: LightCyan on lists border when mode=Lists (REQ-1, AT-1)
  T-2 todo write AT-2 red test: LightCyan on tasks border when mode=Tasks(_) (REQ-2, AT-2)
  T-3 todo write AT-3 red test: color switches on mode toggle (REQ-1, REQ-2, AT-3)
  T-4 todo add Color import and panel_style helper in tui_app.rs (REQ-1, REQ-2, REQ-3)
  T-5 todo apply panel_style to Block in render_lists and render_task_list; make AT-1/AT-2/AT-3 green (REQ-1, REQ-2, REQ-3)

watch-outs:
  - ratatui's Block API: prefer .border_style() + .title_style() for targeted coloring; fall back to .style() if those methods are unavailable in the pinned ratatui version (check Cargo.toml)
  - render_tasks dispatches to render_help / render_editor / render_confirmation in non-None interaction states — those panels do not need focus coloring (out of scope)
  - existing buffer_text helper in tests reads symbols only; color assertions need to use buffer[(x,y)].fg directly — add a helper or inline the assertion
