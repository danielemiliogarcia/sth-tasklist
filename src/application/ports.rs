//! Ports: interfaces the application needs from the outside world, named for
//! capability, not technology. Adapters implement these at the edge.

use crate::domain::task_list::{TaskList, TaskListId};

/// Technology-agnostic persistence error. Adapters map their concrete failures
/// (IO, parse, serialize) into this so the application stays infrastructure-free.
#[derive(Debug)]
pub struct RepoError(pub String);

pub type RepoResult<T> = Result<T, RepoError>;

/// Persistence capability for task lists.
pub trait TaskListRepository {
    fn save(&mut self, list: &TaskList) -> RepoResult<()>;
    fn all(&self) -> RepoResult<Vec<TaskList>>;
    fn by_id(&self, id: &TaskListId) -> RepoResult<Option<TaskList>>;
    fn find_by_name(&self, name: &str) -> RepoResult<Option<TaskList>>;
    fn delete(&mut self, id: &TaskListId) -> RepoResult<()>;
}

/// Identity generation, injected so the domain/use cases stay deterministic.
pub trait IdGenerator {
    fn new_id(&self) -> TaskListId;
}
