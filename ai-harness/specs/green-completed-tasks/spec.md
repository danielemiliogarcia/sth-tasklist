# spec: green-completed-tasks
intent: completed tasks always render their checkmark and title in green, regardless of selection state

requirements:
  REQ-1 (must): a completed task's list item shall be rendered with a foreground colour derived from ColourTheme.completed_task_fg, not from the selection style
  REQ-2 (must): ColourTheme shall expose a completed_task_fg field of type NamedColor with a default of NamedColor::Green
  REQ-3 (must): the TOML theme file shall support an optional completed_task_fg key that overrides the default colour

acceptance:
  AT-1 covers REQ-2 (unit, red):
    Given a ColourTheme built with no explicit completed_task_fg
    When the value is read
    Then completed_task_fg == NamedColor::Green

  AT-2 covers REQ-1 (unit, red):
    Given a ColourTheme with completed_task_fg set to NamedColor::Green
    And a task list containing one completed task and one incomplete task
    When task_items() is called
    Then the completed item's Style has fg == Color::Green
    And the incomplete item's Style does not have fg == Color::Green (unless it is selected and the selection style happens to match — i.e. the style is controlled independently)

  AT-3 covers REQ-3 (integration):
    Given a colours.toml with completed_task_fg = "cyan"
    When TomlThemeRepository loads the theme
    Then the loaded ColourTheme.completed_task_fg == NamedColor::Cyan

  DV-1 covers REQ-1 (manual, visual):
    reason: ratatui rendering cannot be asserted automatically without a test backend harness
    interim check: run the TUI, mark a task complete, confirm the title and checkmark appear green; select the completed task and confirm it remains green
    future path: add a ratatui TestBackend render test once a test-backend helper is in place
    follow-up todo: T-5 (add TestBackend render test)

design:
  domain: ColourTheme (value object) — add completed_task_fg: NamedColor field, default Green; no invariants beyond NamedColor being a closed enum
  ports: ThemeRepository (existing port, no change)
  adapters:
    driven: TomlThemeRepository — add completed_task_fg to ColourThemeDto (Option<String>) and wire through parse_color; missing key defaults to NamedColor::Green
    driving: TuiApp — in task_items(), apply Style::default().fg(named_to_color(&self.theme.completed_task_fg)) to any ListItem where task.is_completed() is true
  usecases: none (pure rendering concern; no use-case orchestration needed)
  feature-local conventions: none — completed_task_fg follows the existing NamedColor/parse_color pattern already used by other ColourTheme fields
