//! Integration test for the TOML-backed task-list repository (AT-8, REQ-6).

use std::fs;
use std::path::PathBuf;

use shtask::adapters::in_memory::SeqIdGenerator;
use shtask::adapters::toml_repo::TomlTaskListRepository;
use shtask::application::create_task_list::CreateTaskList;
use shtask::application::ports::TaskListRepository;

fn unique_temp_dir() -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    std::env::temp_dir().join(format!("shtask-it-{}-{}", std::process::id(), nanos))
}

// AT-8 covers REQ-6: a list created via the TOML repository is present when a
// fresh repository instance loads the same directory.
#[test]
fn persists_and_reloads_from_disk() {
    let dir = unique_temp_dir();
    let ids = SeqIdGenerator::new();

    let id = {
        let mut repo = TomlTaskListRepository::new(&dir);
        let mut uc = CreateTaskList::new(&mut repo, &ids);
        uc.execute("work").unwrap()
    };

    let reloaded = TomlTaskListRepository::new(&dir);
    let names: Vec<String> = reloaded
        .all()
        .unwrap()
        .iter()
        .map(|l| l.name().to_string())
        .collect();

    assert!(names.contains(&"work".to_string()));
    assert!(dir.join(format!("{}.toml", id.as_str())).exists());

    fs::remove_dir_all(&dir).ok();
}
