//! Integration test for the real UUID-backed id generator (AT-1, REQ-1).

use shtask::adapters::uuid_id::UuidIdGenerator;
use shtask::application::ports::IdGenerator;

#[test]
fn generates_non_empty_unique_ids() {
    let ids = UuidIdGenerator::new();

    let first = ids.new_id();
    let second = ids.new_id();

    assert!(!first.as_str().is_empty());
    assert!(!second.as_str().is_empty());
    assert_ne!(first, second);
}
