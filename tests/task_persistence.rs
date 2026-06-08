//! Integration test: tasks and their completion survive a TOML round-trip
//! (AT-15, REQ-3 + REQ-6).

use std::fs;
use std::path::PathBuf;

use shtask::adapters::in_memory::SeqIdGenerator;
use shtask::adapters::toml_repo::TomlTaskListRepository;
use shtask::application::create_task_list::CreateTaskList;
use shtask::application::ports::TaskListRepository;
use shtask::application::task_commands::{AddTask, CompleteTask};

fn unique_temp_dir() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("shtask-tasks-{}-{}", std::process::id(), nanos))
}

#[test]
fn tasks_and_completion_persist_across_reload() {
    let dir = unique_temp_dir();
    let mut repo = TomlTaskListRepository::new(&dir);

    let id = {
        let ids = SeqIdGenerator::new();
        let mut uc = CreateTaskList::new(&mut repo, &ids);
        uc.execute("work").unwrap()
    };
    AddTask::new(&mut repo).execute(&id, "milk").unwrap();
    CompleteTask::new(&mut repo).execute(&id, "milk").unwrap();
    drop(repo);

    let reloaded = TomlTaskListRepository::new(&dir);
    let list = reloaded.by_id(&id).unwrap().expect("list reloaded");

    assert_eq!(list.tasks().len(), 1);
    assert_eq!(list.tasks()[0].title(), "milk");
    assert!(list.tasks()[0].is_completed());
    assert!(list.is_completed());

    fs::remove_dir_all(&dir).ok();
}
