//! The `TaskList` aggregate root and its invariants. Pure — no IO, no serde.

use crate::domain::task::{Task, TaskError};

/// Stable identity of a task list. The on-disk file name derives from this.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TaskListId(pub String);

impl TaskListId {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors raised by the task-list domain invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DomainError {
    /// A list name was empty or whitespace-only.
    EmptyName,
}

/// A named collection of tasks, and the aggregate root that owns them.
/// Invariants: `name` is non-empty/non-whitespace; task titles are unique within
/// the list. Uniqueness of list *names* across lists is a use-case rule, not here.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskList {
    id: TaskListId,
    name: String,
    tasks: Vec<Task>,
}

impl TaskList {
    /// Build an empty task list, enforcing the non-empty name invariant.
    pub fn new(id: TaskListId, name: &str) -> Result<Self, DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::EmptyName);
        }
        Ok(Self {
            id,
            name: name.to_string(),
            tasks: Vec::new(),
        })
    }

    pub fn id(&self) -> &TaskListId {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    /// Change the name, re-enforcing the non-empty invariant.
    pub fn rename(&mut self, name: &str) -> Result<(), DomainError> {
        if name.trim().is_empty() {
            return Err(DomainError::EmptyName);
        }
        self.name = name.to_string();
        Ok(())
    }

    /// The list's tasks, in insertion order.
    pub fn tasks(&self) -> &[Task] {
        &self.tasks
    }

    /// Add a task, enforcing a non-empty title unique within this list.
    pub fn add_task(&mut self, title: &str) -> Result<(), TaskError> {
        let task = Task::new(title)?;
        if self.tasks.iter().any(|t| t.title() == task.title()) {
            return Err(TaskError::DuplicateTitle);
        }
        self.tasks.push(task);
        Ok(())
    }

    /// Rename the task titled `old_title`, keeping titles non-empty and unique.
    /// Renaming a task to its own current title is a no-op and allowed.
    pub fn rename_task(&mut self, old_title: &str, new_title: &str) -> Result<(), TaskError> {
        if new_title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        let idx = self
            .tasks
            .iter()
            .position(|t| t.title() == old_title)
            .ok_or(TaskError::NotFound)?;
        if self
            .tasks
            .iter()
            .enumerate()
            .any(|(i, t)| i != idx && t.title() == new_title)
        {
            return Err(TaskError::DuplicateTitle);
        }
        self.tasks[idx].set_title(new_title.to_string());
        Ok(())
    }

    /// Remove the task titled `title`. Reports NotFound if no such task exists.
    pub fn remove_task(&mut self, title: &str) -> Result<(), TaskError> {
        let idx = self
            .tasks
            .iter()
            .position(|t| t.title() == title)
            .ok_or(TaskError::NotFound)?;
        self.tasks.remove(idx);
        Ok(())
    }

    /// Mark the task titled `title` completed (idempotent).
    pub fn complete_task(&mut self, title: &str) -> Result<(), TaskError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.title() == title)
            .ok_or(TaskError::NotFound)?;
        task.mark_completed();
        Ok(())
    }

    /// Mark the task titled `title` incomplete (idempotent). Reports NotFound if absent.
    pub fn uncomplete_task(&mut self, title: &str) -> Result<(), TaskError> {
        let task = self
            .tasks
            .iter_mut()
            .find(|t| t.title() == title)
            .ok_or(TaskError::NotFound)?;
        task.mark_uncompleted();
        Ok(())
    }

    /// A list is completed iff it has at least one task and every task is
    /// completed. An empty list is *not* completed.
    pub fn is_completed(&self) -> bool {
        !self.tasks.is_empty() && self.tasks.iter().all(|t| t.is_completed())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_whitespace_only_name() {
        assert_eq!(
            TaskList::new(TaskListId("x".into()), "   "),
            Err(DomainError::EmptyName)
        );
    }

    #[test]
    fn keeps_name_and_id_when_valid() {
        let list = TaskList::new(TaskListId("x".into()), "work").unwrap();
        assert_eq!(list.name(), "work");
        assert_eq!(list.id().as_str(), "x");
    }

    fn list() -> TaskList {
        TaskList::new(TaskListId("x".into()), "work").unwrap()
    }

    // AT-1 covers REQ-1: a blank title is rejected and no task is added.
    #[test]
    fn add_task_rejects_blank_title() {
        let mut l = list();
        assert_eq!(l.add_task("   "), Err(TaskError::EmptyTitle));
        assert!(l.tasks().is_empty());
    }

    // AT-2 covers REQ-1: adding a title yields one incomplete task.
    #[test]
    fn add_task_appends_incomplete_task() {
        let mut l = list();
        l.add_task("milk").unwrap();
        assert_eq!(l.tasks().len(), 1);
        assert_eq!(l.tasks()[0].title(), "milk");
        assert!(!l.tasks()[0].is_completed());
    }

    // AT-3 covers REQ-2: a duplicate title in the same list is rejected.
    #[test]
    fn add_task_rejects_duplicate_title() {
        let mut l = list();
        l.add_task("milk").unwrap();
        assert_eq!(l.add_task("milk"), Err(TaskError::DuplicateTitle));
        assert_eq!(l.tasks().len(), 1);
    }

    // AT-5 covers REQ-4: a task can be renamed.
    #[test]
    fn rename_task_changes_title() {
        let mut l = list();
        l.add_task("milk").unwrap();
        l.rename_task("milk", "bread").unwrap();
        assert_eq!(l.tasks()[0].title(), "bread");
    }

    // AT-6 covers REQ-4: renaming onto another task's title is rejected.
    #[test]
    fn rename_task_rejects_duplicate_title() {
        let mut l = list();
        l.add_task("milk").unwrap();
        l.add_task("bread").unwrap();
        assert_eq!(
            l.rename_task("bread", "milk"),
            Err(TaskError::DuplicateTitle)
        );
        assert_eq!(l.tasks()[1].title(), "bread");
    }

    // AT-7 covers REQ-4: renaming a missing task reports NotFound.
    #[test]
    fn rename_missing_task_is_not_found() {
        let mut l = list();
        assert_eq!(l.rename_task("ghost", "x"), Err(TaskError::NotFound));
    }

    // AT-8 covers REQ-5: a task can be removed.
    #[test]
    fn remove_task_drops_it() {
        let mut l = list();
        l.add_task("milk").unwrap();
        l.remove_task("milk").unwrap();
        assert!(l.tasks().is_empty());
    }

    // AT-9 covers REQ-5: removing a missing task reports NotFound.
    #[test]
    fn remove_missing_task_is_not_found() {
        let mut l = list();
        assert_eq!(l.remove_task("ghost"), Err(TaskError::NotFound));
    }

    // AT-10 covers REQ-6: completing an incomplete task marks it complete.
    #[test]
    fn complete_task_marks_completed() {
        let mut l = list();
        l.add_task("milk").unwrap();
        l.complete_task("milk").unwrap();
        assert!(l.tasks()[0].is_completed());
    }

    // AT-11 covers REQ-6: re-completing a completed task is a no-op (no error).
    #[test]
    fn complete_task_is_idempotent() {
        let mut l = list();
        l.add_task("milk").unwrap();
        l.complete_task("milk").unwrap();
        l.complete_task("milk").unwrap();
        assert!(l.tasks()[0].is_completed());
    }

    // AT-12 covers REQ-7: completed only when every task is completed.
    #[test]
    fn list_completes_when_all_tasks_complete() {
        let mut l = list();
        l.add_task("milk").unwrap();
        l.add_task("bread").unwrap();
        l.complete_task("milk").unwrap();
        assert!(!l.is_completed());
        l.complete_task("bread").unwrap();
        assert!(l.is_completed());
    }

    // AT-13 covers REQ-7: an empty list is not completed.
    #[test]
    fn empty_list_is_not_completed() {
        assert!(!list().is_completed());
    }

    // AT-2 covers REQ-2 (uncomplete-task): uncomplete_task resets a completed task
    #[test]
    fn uncomplete_task_resets_completed_task() {
        let mut l = list();
        l.add_task("milk").unwrap();
        l.complete_task("milk").unwrap();
        assert!(l.tasks()[0].is_completed());
        l.uncomplete_task("milk").unwrap();
        assert!(!l.tasks()[0].is_completed());
    }

    // AT-3 covers REQ-2 (uncomplete-task): uncomplete_task returns NotFound for unknown title
    #[test]
    fn uncomplete_missing_task_is_not_found() {
        let mut l = list();
        assert_eq!(l.uncomplete_task("ghost"), Err(TaskError::NotFound));
    }
}
