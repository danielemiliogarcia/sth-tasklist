//! The `Task` value object and task-operation errors. Pure — no IO, no serde.

/// Errors from task operations on a list aggregate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TaskError {
    /// A task title was empty or whitespace-only.
    EmptyTitle,
    /// A task with the same title already exists in the list.
    DuplicateTitle,
    /// No task with the given title exists in the list.
    NotFound,
}

/// A single task. Invariant: `title` is non-empty/non-whitespace. New tasks
/// start incomplete. Completion is one-way (see the aggregate's `complete_task`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    title: String,
    completed: bool,
}

impl Task {
    /// Create an incomplete task, enforcing the non-empty title invariant.
    pub fn new(title: &str) -> Result<Self, TaskError> {
        if title.trim().is_empty() {
            return Err(TaskError::EmptyTitle);
        }
        Ok(Self {
            title: title.to_string(),
            completed: false,
        })
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Change the title. Crate-internal: the `TaskList` aggregate validates the
    /// new title (non-empty, unique) before calling this.
    pub(crate) fn set_title(&mut self, title: String) {
        self.title = title;
    }

    /// Mark the task completed. One-way and idempotent — completing an already
    /// completed task leaves it completed.
    pub(crate) fn mark_completed(&mut self) {
        self.completed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_blank_title() {
        assert_eq!(Task::new("   "), Err(TaskError::EmptyTitle));
    }

    #[test]
    fn new_task_starts_incomplete() {
        let t = Task::new("milk").unwrap();
        assert_eq!(t.title(), "milk");
        assert!(!t.is_completed());
    }
}
