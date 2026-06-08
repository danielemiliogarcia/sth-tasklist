//! In-memory adapters: a repository fake and a deterministic id generator.
//! No IO — safe to use directly in unit tests.

use std::cell::Cell;

use crate::application::ports::{IdGenerator, RepoResult, TaskListRepository};
use crate::domain::task_list::{TaskList, TaskListId};

#[derive(Default)]
pub struct InMemoryTaskListRepository {
    lists: Vec<TaskList>,
}

impl InMemoryTaskListRepository {
    pub fn new() -> Self {
        Self::default()
    }
}

impl TaskListRepository for InMemoryTaskListRepository {
    fn save(&mut self, list: &TaskList) -> RepoResult<()> {
        match self.lists.iter_mut().find(|l| l.id() == list.id()) {
            Some(existing) => *existing = list.clone(),
            None => self.lists.push(list.clone()),
        }
        Ok(())
    }

    fn all(&self) -> RepoResult<Vec<TaskList>> {
        Ok(self.lists.clone())
    }

    fn by_id(&self, id: &TaskListId) -> RepoResult<Option<TaskList>> {
        Ok(self.lists.iter().find(|l| l.id() == id).cloned())
    }

    fn find_by_name(&self, name: &str) -> RepoResult<Option<TaskList>> {
        Ok(self.lists.iter().find(|l| l.name() == name).cloned())
    }

    fn delete(&mut self, id: &TaskListId) -> RepoResult<()> {
        self.lists.retain(|l| l.id() != id);
        Ok(())
    }
}

/// Deterministic id generator for tests: `seq-0`, `seq-1`, …
pub struct SeqIdGenerator {
    next: Cell<u64>,
}

impl SeqIdGenerator {
    pub fn new() -> Self {
        Self { next: Cell::new(0) }
    }
}

impl Default for SeqIdGenerator {
    fn default() -> Self {
        Self::new()
    }
}

impl IdGenerator for SeqIdGenerator {
    fn new_id(&self) -> TaskListId {
        let n = self.next.get();
        self.next.set(n + 1);
        TaskListId(format!("seq-{n}"))
    }
}
