//! Use case: create a task list. Validates the name, then persists via the port.

use crate::application::ports::{IdGenerator, TaskListRepository};
use crate::domain::task_list::{DomainError, TaskList, TaskListId};

#[derive(Debug, PartialEq, Eq)]
pub enum CreateError {
    EmptyName,
    DuplicateName,
    Repo(String),
}

impl From<DomainError> for CreateError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::EmptyName => CreateError::EmptyName,
        }
    }
}

pub struct CreateTaskList<'a, R: TaskListRepository, I: IdGenerator> {
    repo: &'a mut R,
    ids: &'a I,
}

impl<'a, R: TaskListRepository, I: IdGenerator> CreateTaskList<'a, R, I> {
    pub fn new(repo: &'a mut R, ids: &'a I) -> Self {
        Self { repo, ids }
    }

    /// Build a validated `TaskList` and persist it, returning its id.
    /// Rejects an empty name (domain invariant) or a name already in use.
    pub fn execute(&mut self, name: &str) -> Result<TaskListId, CreateError> {
        let id = self.ids.new_id();
        let list = TaskList::new(id, name)?;
        if self
            .repo
            .find_by_name(list.name())
            .map_err(|e| CreateError::Repo(e.0))?
            .is_some()
        {
            return Err(CreateError::DuplicateName);
        }
        self.repo.save(&list).map_err(|e| CreateError::Repo(e.0))?;
        Ok(list.id().clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::in_memory::{InMemoryTaskListRepository, SeqIdGenerator};

    // AT-1 covers REQ-1: a blank name is rejected and nothing is stored.
    #[test]
    fn create_rejects_blank_name() {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        let mut uc = CreateTaskList::new(&mut repo, &ids);

        let err = uc.execute("   ").unwrap_err();

        assert_eq!(err, CreateError::EmptyName);
        assert!(repo.all().unwrap().is_empty());
    }

    // AT-2 covers REQ-1: a valid name yields a stored list with a new id.
    #[test]
    fn create_persists_named_list() {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        let id = {
            let mut uc = CreateTaskList::new(&mut repo, &ids);
            uc.execute("work").unwrap()
        };

        let stored = repo.by_id(&id).unwrap().expect("list stored");
        assert_eq!(stored.name(), "work");
        assert_eq!(stored.id(), &id);
    }

    // AT-3 covers REQ-2: a name already in use is rejected.
    #[test]
    fn create_rejects_duplicate_name() {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        {
            let mut uc = CreateTaskList::new(&mut repo, &ids);
            uc.execute("work").unwrap();
        }

        let err = {
            let mut uc = CreateTaskList::new(&mut repo, &ids);
            uc.execute("work").unwrap_err()
        };

        assert_eq!(err, CreateError::DuplicateName);
        assert_eq!(repo.all().unwrap().len(), 1);
    }
}
