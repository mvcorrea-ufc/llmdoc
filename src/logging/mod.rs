// llmdoc/src/logging/mod.rs
//! Logging module for the application.
//!
//! This module provides a flexible and configurable logging facade.
//! It includes a `Logger` for emitting log messages and a `FileWriter`
//! for handling log persistence with features like rotation and cleanup.

pub mod file_writer;
pub mod logger;

pub use logger::{Logger, LogLevel, LogConfig};
// Potentially re-export macros if they are defined in logger.rs
// pub use logger::{log_trace, log_debug, log_info, log_warn, log_error, log_fatal};