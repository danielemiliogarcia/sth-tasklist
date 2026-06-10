//! Binary entry point and composition root.
//!
//! Concrete adapters are constructed here, at the edge, and injected into the
//! application use cases. The TUI driving adapter is added in the next slice.

use std::process::ExitCode;

use shtask::adapters::toml_repo::TomlTaskListRepository;
use shtask::adapters::toml_theme::TomlThemeRepository;
use shtask::adapters::tui_app::App;
use shtask::adapters::uuid_id::UuidIdGenerator;
use shtask::application::create_task_list::CreateTaskList;
use shtask::application::delete_task_list::DeleteTaskList;
use shtask::application::list_task_lists::ListTaskLists;
use shtask::application::ports::ThemeRepository;
use shtask::application::rename_task_list::RenameTaskList;
use shtask::application::task_commands::{
    AddTask, CompleteTask, DeleteTask, ListTasks, RenameTask,
};

const DATA_DIR: &str = ".shtask";

fn main() -> ExitCode {
    match run(std::env::args().skip(1)) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("shtask: {e}");
            ExitCode::FAILURE
        }
    }
}

fn run(args: impl IntoIterator<Item = String>) -> Result<(), String> {
    let headless = args.into_iter().any(|arg| arg == "--headless");
    let mut repo = TomlTaskListRepository::new(DATA_DIR);
    let ids = UuidIdGenerator::new();
    wire_usecases(&mut repo, &ids)?;

    // colours.toml is optional; missing/malformed file falls back to defaults
    let theme = TomlThemeRepository::new("./colours.toml").load();
    let mut app = App::new(repo, ids, theme);
    if headless {
        return app.load_once();
    }

    app.run().map_err(|e| e.to_string())
}

fn wire_usecases(repo: &mut TomlTaskListRepository, ids: &UuidIdGenerator) -> Result<(), String> {
    {
        let _create_task_list = CreateTaskList::new(repo, ids);
    }

    let list_count = {
        let list_task_lists = ListTaskLists::new(&*repo);
        list_task_lists.execute().map_err(|e| e.0)?.len()
    };

    {
        let _rename_task_list = RenameTaskList::new(repo);
    }
    {
        let _delete_task_list = DeleteTaskList::new(repo);
    }
    {
        let _add_task = AddTask::new(repo);
    }
    {
        let _rename_task = RenameTask::new(repo);
    }
    {
        let _delete_task = DeleteTask::new(repo);
    }
    {
        let _complete_task = CompleteTask::new(repo);
    }
    {
        let _list_tasks = ListTasks::new(&*repo);
    }

    let _ = list_count;
    Ok(())
}
