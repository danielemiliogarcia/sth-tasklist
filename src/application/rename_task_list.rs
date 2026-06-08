//! Use case: rename an existing task list, keeping names non-empty and unique.

use crate::application::ports::TaskListRepository;
use crate::domain::task_list::{DomainError, TaskListId};

#[derive(Debug, PartialEq, Eq)]
pub enum RenameError {
    NotFound,
    EmptyName,
    DuplicateName,
    Repo(String),
}

impl From<DomainError> for RenameError {
    fn from(e: DomainError) -> Self {
        match e {
            DomainError::EmptyName => RenameError::EmptyName,
        }
    }
}

pub struct RenameTaskList<'a, R: TaskListRepository> {
    repo: &'a mut R,
}

impl<'a, R: TaskListRepository> RenameTaskList<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, id: &TaskListId, new_name: &str) -> Result<(), RenameError> {
        let mut list = self
            .repo
            .by_id(id)
            .map_err(|e| RenameError::Repo(e.0))?
            .ok_or(RenameError::NotFound)?;

        // A name in use by a *different* list is a conflict; renaming to its own
        // current name is a no-op and allowed.
        if let Some(other) = self
            .repo
            .find_by_name(new_name)
            .map_err(|e| RenameError::Repo(e.0))?
            && other.id() != id
        {
            return Err(RenameError::DuplicateName);
        }

        list.rename(new_name)?;
        self.repo.save(&list).map_err(|e| RenameError::Repo(e.0))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::in_memory::{InMemoryTaskListRepository, SeqIdGenerator};
    use crate::application::create_task_list::CreateTaskList;

    fn seed(repo: &mut InMemoryTaskListRepository, ids: &SeqIdGenerator, name: &str) -> TaskListId {
        let mut uc = CreateTaskList::new(repo, ids);
        uc.execute(name).unwrap()
    }

    // AT-5 covers REQ-4: an existing list can be renamed.
    #[test]
    fn renames_existing_list() {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        let id = seed(&mut repo, &ids, "work");

        RenameTaskList::new(&mut repo)
            .execute(&id, "office")
            .unwrap();

        assert_eq!(repo.by_id(&id).unwrap().unwrap().name(), "office");
    }

    // AT-6 covers REQ-4: renaming onto another list's name is rejected.
    #[test]
    fn rejects_rename_to_existing_name() {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        seed(&mut repo, &ids, "work");
        let home = seed(&mut repo, &ids, "home");

        let err = RenameTaskList::new(&mut repo)
            .execute(&home, "work")
            .unwrap_err();

        assert_eq!(err, RenameError::DuplicateName);
        assert_eq!(repo.by_id(&home).unwrap().unwrap().name(), "home");
    }

    #[test]
    fn missing_list_is_not_found() {
        let mut repo = InMemoryTaskListRepository::new();
        let err = RenameTaskList::new(&mut repo)
            .execute(&TaskListId("nope".into()), "x")
            .unwrap_err();
        assert_eq!(err, RenameError::NotFound);
    }
}
