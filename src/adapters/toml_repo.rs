//! Driven adapter: persist task lists as TOML, one `<id>.toml` file per list.
//! A `TaskListDto` translates between the on-disk shape and the pure domain at
//! this boundary, keeping the domain serde-free.

use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::application::ports::{RepoError, RepoResult, TaskListRepository};
use crate::domain::task_list::{TaskList, TaskListId};

#[derive(Serialize, Deserialize)]
struct TaskDto {
    title: String,
    completed: bool,
}

#[derive(Serialize, Deserialize)]
struct TaskListDto {
    id: String,
    name: String,
    // Absent in files written before tasks existed -> defaults to empty.
    #[serde(default)]
    tasks: Vec<TaskDto>,
}

impl TaskListDto {
    fn from_domain(list: &TaskList) -> Self {
        Self {
            id: list.id().as_str().to_string(),
            name: list.name().to_string(),
            tasks: list
                .tasks()
                .iter()
                .map(|t| TaskDto {
                    title: t.title().to_string(),
                    completed: t.is_completed(),
                })
                .collect(),
        }
    }

    fn into_domain(self) -> RepoResult<TaskList> {
        let mut list = TaskList::new(TaskListId(self.id), &self.name)
            .map_err(|e| RepoError(format!("invalid stored task list: {e:?}")))?;
        // Rebuild tasks through the aggregate so stored data re-enforces invariants.
        for t in self.tasks {
            list.add_task(&t.title)
                .map_err(|e| RepoError(format!("invalid stored task: {e:?}")))?;
            if t.completed {
                list.complete_task(&t.title)
                    .map_err(|e| RepoError(format!("invalid stored task: {e:?}")))?;
            }
        }
        Ok(list)
    }
}

pub struct TomlTaskListRepository {
    dir: PathBuf,
}

impl TomlTaskListRepository {
    /// Repository rooted at `dir` (e.g. `./.shtask`); created on first write.
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    fn path_for(&self, id: &TaskListId) -> PathBuf {
        self.dir.join(format!("{}.toml", id.as_str()))
    }
}

impl TaskListRepository for TomlTaskListRepository {
    fn save(&mut self, list: &TaskList) -> RepoResult<()> {
        fs::create_dir_all(&self.dir).map_err(|e| RepoError(e.to_string()))?;
        let text = toml::to_string(&TaskListDto::from_domain(list))
            .map_err(|e| RepoError(e.to_string()))?;
        fs::write(self.path_for(list.id()), text).map_err(|e| RepoError(e.to_string()))
    }

    fn all(&self) -> RepoResult<Vec<TaskList>> {
        let entries = match fs::read_dir(&self.dir) {
            Ok(e) => e,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
            Err(e) => return Err(RepoError(e.to_string())),
        };
        let mut out = Vec::new();
        for entry in entries {
            let path = entry.map_err(|e| RepoError(e.to_string()))?.path();
            if path.extension().and_then(|s| s.to_str()) == Some("toml") {
                out.push(read_list(&path)?);
            }
        }
        Ok(out)
    }

    fn by_id(&self, id: &TaskListId) -> RepoResult<Option<TaskList>> {
        let path = self.path_for(id);
        if !path.exists() {
            return Ok(None);
        }
        Ok(Some(read_list(&path)?))
    }

    fn find_by_name(&self, name: &str) -> RepoResult<Option<TaskList>> {
        Ok(self.all()?.into_iter().find(|l| l.name() == name))
    }

    fn delete(&mut self, id: &TaskListId) -> RepoResult<()> {
        match fs::remove_file(self.path_for(id)) {
            Ok(()) => Ok(()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(e) => Err(RepoError(e.to_string())),
        }
    }
}

fn read_list(path: &Path) -> RepoResult<TaskList> {
    let text = fs::read_to_string(path).map_err(|e| RepoError(e.to_string()))?;
    let dto: TaskListDto = toml::from_str(&text).map_err(|e| RepoError(e.to_string()))?;
    dto.into_domain()
}
