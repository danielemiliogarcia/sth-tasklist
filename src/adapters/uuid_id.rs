//! Driven adapter: random UUID-backed id generation for real application runs.

use uuid::Uuid;

use crate::application::ports::IdGenerator;
use crate::domain::task_list::TaskListId;

#[derive(Default)]
pub struct UuidIdGenerator;

impl UuidIdGenerator {
    pub fn new() -> Self {
        Self
    }
}

impl IdGenerator for UuidIdGenerator {
    fn new_id(&self) -> TaskListId {
        TaskListId(Uuid::new_v4().to_string())
    }
}
