//! shtask — terminal task-list manager.
//!
//! Hexagonal layout: `domain` is pure, `application` declares ports and
//! orchestrates use cases, `adapters` implement the ports at the edge.

pub mod adapters;
pub mod application;
pub mod domain;
