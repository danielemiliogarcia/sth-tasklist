//! Documentation check for current TUI launch and key bindings (AT-11, REQ-12).

#[test]
fn readme_documents_tui_launch_and_keys() {
    let readme = include_str!("../README.md");

    for expected in [
        "cargo run",
        "?",
        "q",
        "Esc",
        "List mode",
        "Task mode",
        "n",
        "r",
        "d",
        "Space",
    ] {
        assert!(
            readme.contains(expected),
            "README.md should document `{expected}`"
        );
    }
}
