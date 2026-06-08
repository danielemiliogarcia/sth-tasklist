# shtask

Terminal task-list manager backed by local TOML files in `./.shtask/`.

## Run

```bash
cargo run
```

For a non-interactive startup check:

```bash
cargo run -- --headless
```

## TUI Keys

Press `?` in the app to show help for the active mode. Press `q` to quit.
The selected row is marked with `>>`, and the status line also shows the current
selection, such as `List 1/2: work` or `Task 2/3: milk`.

List mode:

- `n` new list
- `r` rename selected list
- `d` delete selected list, then `y` to confirm
- `Enter` open selected list's tasks
- `Up` / `Down` select a list; selection wraps at the top and bottom
- `Esc` closes help or cancels prompts

Task mode:

- `n` new task
- `r` rename selected task
- `d` delete selected task, then `y` to confirm
- `Space` complete selected task
- `Up` / `Down` select a task; selection wraps at the top and bottom
- `Esc` returns to List mode, closes help, or cancels prompts
