//! Use case: return every task list.

use crate::application::ports::{RepoError, TaskListRepository};
use crate::domain::task_list::TaskList;

pub struct ListTaskLists<'a, R: TaskListRepository> {
    repo: &'a R,
}

impl<'a, R: TaskListRepository> ListTaskLists<'a, R> {
    pub fn new(repo: &'a R) -> Self {
        Self { repo }
    }

    pub fn execute(&self) -> Result<Vec<TaskList>, RepoError> {
        self.repo.all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::in_memory::{InMemoryTaskListRepository, SeqIdGenerator};
    use crate::application::create_task_list::CreateTaskList;

    // AT-4 covers REQ-3: every existing list is returned.
    #[test]
    fn lists_all_existing() {
        let mut repo = InMemoryTaskListRepository::new();
        let ids = SeqIdGenerator::new();
        for name in ["work", "home"] {
            let mut uc = CreateTaskList::new(&mut repo, &ids);
            uc.execute(name).unwrap();
        }

        let names: Vec<String> = ListTaskLists::new(&repo)
            .execute()
            .unwrap()
            .iter()
            .map(|l| l.name().to_string())
            .collect();

        assert_eq!(names, vec!["work".to_string(), "home".to_string()]);
    }
}
