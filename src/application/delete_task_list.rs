//! Use case: delete an existing task list.

use crate::application::ports::TaskListRepository;
use crate::domain::task_list::TaskListId;

#[derive(Debug, PartialEq, Eq)]
pub enum DeleteError {
    NotFound,
    Repo(String),
}

pub struct DeleteTaskList<'a, R: TaskListRepository> {
    repo: &'a mut R,
}

impl<'a, R: TaskListRepository> DeleteTaskList<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, id: &TaskListId) -> Result<(), DeleteError> {
        if self
            .repo
            .by_id(id)
            .map_err(|e| DeleteError::Repo(e.0))?
            .is_none()
        {
            return Err(DeleteError::NotFound);
        }
        self.repo.delete(id).map_err(|e| DeleteError::Repo(e.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::in_memory::{InMemoryTaskListRepository, SeqIdGenerator};
    use crate::application::create_task_list::CreateTaskList;

    // AT-7 covers REQ-5: a deleted list is no longer listed.
    #[test]
    fn deletes_existing_list() {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        let id = {
            let mut uc = CreateTaskList::new(&mut repo, &ids);
            uc.execute("work").unwrap()
        };

        DeleteTaskList::new(&mut repo).execute(&id).unwrap();

        assert!(repo.all().unwrap().is_empty());
    }

    #[test]
    fn missing_list_is_not_found() {
        let mut repo = InMemoryTaskListRepository::new();
        let err = DeleteTaskList::new(&mut repo)
            .execute(&TaskListId("nope".into()))
            .unwrap_err();
        assert_eq!(err, DeleteError::NotFound);
    }
}
