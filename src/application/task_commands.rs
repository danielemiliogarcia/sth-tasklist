//! Use cases that operate on the tasks inside a list. Each loads the list via
//! the repository port, mutates the aggregate, and saves it back.

use crate::application::ports::TaskListRepository;
use crate::domain::task::{Task, TaskError};
use crate::domain::task_list::TaskListId;

/// Error shared by the task command use cases.
#[derive(Debug, PartialEq, Eq)]
pub enum TaskCommandError {
    ListNotFound,
    EmptyTitle,
    DuplicateTitle,
    TaskNotFound,
    Repo(String),
}

impl From<TaskError> for TaskCommandError {
    fn from(e: TaskError) -> Self {
        match e {
            TaskError::EmptyTitle => Self::EmptyTitle,
            TaskError::DuplicateTitle => Self::DuplicateTitle,
            TaskError::NotFound => Self::TaskNotFound,
        }
    }
}

/// Add a task to an existing list.
pub struct AddTask<'a, R: TaskListRepository> {
    repo: &'a mut R,
}

impl<'a, R: TaskListRepository> AddTask<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, list_id: &TaskListId, title: &str) -> Result<(), TaskCommandError> {
        let mut list = self
            .repo
            .by_id(list_id)
            .map_err(|e| TaskCommandError::Repo(e.0))?
            .ok_or(TaskCommandError::ListNotFound)?;
        list.add_task(title)?;
        self.repo
            .save(&list)
            .map_err(|e| TaskCommandError::Repo(e.0))
    }
}

/// Rename a task within an existing list.
pub struct RenameTask<'a, R: TaskListRepository> {
    repo: &'a mut R,
}

impl<'a, R: TaskListRepository> RenameTask<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(
        &mut self,
        list_id: &TaskListId,
        old_title: &str,
        new_title: &str,
    ) -> Result<(), TaskCommandError> {
        let mut list = self
            .repo
            .by_id(list_id)
            .map_err(|e| TaskCommandError::Repo(e.0))?
            .ok_or(TaskCommandError::ListNotFound)?;
        list.rename_task(old_title, new_title)?;
        self.repo
            .save(&list)
            .map_err(|e| TaskCommandError::Repo(e.0))
    }
}

/// Delete a task from an existing list.
pub struct DeleteTask<'a, R: TaskListRepository> {
    repo: &'a mut R,
}

impl<'a, R: TaskListRepository> DeleteTask<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, list_id: &TaskListId, title: &str) -> Result<(), TaskCommandError> {
        let mut list = self
            .repo
            .by_id(list_id)
            .map_err(|e| TaskCommandError::Repo(e.0))?
            .ok_or(TaskCommandError::ListNotFound)?;
        list.remove_task(title)?;
        self.repo
            .save(&list)
            .map_err(|e| TaskCommandError::Repo(e.0))
    }
}

/// Mark a task completed within an existing list (one-way, idempotent).
pub struct CompleteTask<'a, R: TaskListRepository> {
    repo: &'a mut R,
}

impl<'a, R: TaskListRepository> CompleteTask<'a, R> {
    pub fn new(repo: &'a mut R) -> Self {
        Self { repo }
    }

    pub fn execute(&mut self, list_id: &TaskListId, title: &str) -> Result<(), TaskCommandError> {
        let mut list = self
            .repo
            .by_id(list_id)
            .map_err(|e| TaskCommandError::Repo(e.0))?
            .ok_or(TaskCommandError::ListNotFound)?;
        list.complete_task(title)?;
        self.repo
            .save(&list)
            .map_err(|e| TaskCommandError::Repo(e.0))
    }
}

/// Read the tasks of an existing list, in insertion order.
pub struct ListTasks<'a, R: TaskListRepository> {
    repo: &'a R,
}

impl<'a, R: TaskListRepository> ListTasks<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub fn execute(&self, list_id: &TaskListId) -> Result<Vec<Task>, TaskCommandError> {
        let list = self
            .repo
            .by_id(list_id)
            .map_err(|e| TaskCommandError::Repo(e.0))?
            .ok_or(TaskCommandError::ListNotFound)?;
        Ok(list.tasks().to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::in_memory::{InMemoryTaskListRepository, SeqIdGenerator};
    use crate::application::create_task_list::CreateTaskList;

    fn repo_with_list() -> (InMemoryTaskListRepository, TaskListId) {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        let id = {
            let mut uc = CreateTaskList::new(&mut repo, &ids);
            uc.execute("work").unwrap()
        };
        (repo, id)
    }

    // AT-4 covers REQ-3: tasks are returned in insertion order.
    #[test]
    fn lists_tasks_in_insertion_order() {
        let (mut repo, id) = repo_with_list();
        for title in ["a", "b"] {
            AddTask::new(&mut repo).execute(&id, title).unwrap();
        }

        let titles: Vec<String> = ListTasks::new(&repo)
            .execute(&id)
            .unwrap()
            .iter()
            .map(|t| t.title().to_string())
            .collect();

        assert_eq!(titles, vec!["a".to_string(), "b".to_string()]);
    }

    // AT-14 covers REQ-8: adding to a missing list reports ListNotFound.
    #[test]
    fn add_to_missing_list_is_not_found() {
        let mut repo = InMemoryTaskListRepository::new();
        let err = AddTask::new(&mut repo)
            .execute(&TaskListId("nope".into()), "milk")
            .unwrap_err();
        assert_eq!(err, TaskCommandError::ListNotFound);
    }
}
