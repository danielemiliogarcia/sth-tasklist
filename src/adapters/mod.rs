//! Edge implementations of the application ports.
//!
//! `in_memory` holds pure, IO-free doubles used by unit tests and as a simple
//! reference implementation. The TOML-backed driven adapter lands in a later task.

pub mod in_memory;
pub mod toml_repo;
pub mod toml_theme;
pub mod tui_app;
pub mod uuid_id;
