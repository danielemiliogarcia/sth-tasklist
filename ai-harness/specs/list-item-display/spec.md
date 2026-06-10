# spec: list-item-display
intent: replace "pending" with ☐ for uncompleted task lists and remove redundant custom selection markers so items do not shift position on selection
# how to fill + legal forms + IDs: see ../global-spec-info.md

requirements:
  REQ-1 (must): an uncompleted task list displays "☐" (empty checkbox) instead of the word "pending"
  REQ-2 (must): list items and task items do not change horizontal position when selected; selection is indicated by ratatui's highlight style alone (no custom >> / spaces prefix that shifts text)

scope:
  in: render_lists, task_items(), preview_task_items() in tui_app.rs
  out: completed item rendering; highlight colors; any domain change
assumptions:
  - ratatui's highlight_symbol ("> ") + highlight_style already handle selection visibility
  - removing the custom >> / "  " marker entirely is the correct fix
open questions:
  - none

acceptance:
  AT-1 covers REQ-1 (adapter unit, red):
    Given app in Lists mode with an uncompleted list / When rendered / Then right pane contains "☐" and does not contain "pending"

  AT-2 covers REQ-2 (adapter unit, red):
    Given app in Lists mode with multiple lists / When item is selected vs not selected / Then the task title appears at the same column offset in both states (no >> prefix shift)

  AT-3 covers REQ-2 (adapter unit, red):
    Given app in Tasks mode / When a task is selected vs not selected / Then the task title appears at the same column offset in both states

design:
  domain: no changes
  ports: no changes
  adapters:
    driving: in render_lists — remove `marker` variable; change format to "{status} {name}" where status is "☐" for pending and "✓" for completed
    driving: in task_items() — remove `marker` variable; change format to "{status} {title}"
    driving: in preview_task_items() — change format from "   {status} {title}" to "{status} {title}"
  usecases: none
  feature-local conventions: none — item text is now "{status} {title}" with no positional prefix
