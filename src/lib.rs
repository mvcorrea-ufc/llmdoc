//! # LLMDocs - LLM-Powered Documentation Management System
//!
//! This crate provides a comprehensive solution for managing Agile development documentation
//! using Large Language Models. It includes task tracking, sprint management, ADR handling,
//! user story management, and more.
//!
//! ## Core Modules
//!
//! - [`core`] - Core functionality including database operations, models, and error handling
//! - [`services`] - Business logic services for each entity type
//! - [`cli`] - Command-line interface implementation
//! - [`migration`] - Tools for migrating from existing markdown documentation
//! - [`embeddings`] - Text embedding abstractions for semantic search
//! - [`export`] - Data export functionality to various formats
//! - [`logging`] - Structured logging setup and utilities
//! - [`app_config`] - Application configuration management
//!
//! ## Quick Start
//!
//! ```no_run
//! use llmdocs::{Config, initialize_app};
//!
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     // Initialize the application
//!     initialize_app().await?;
//!     
//!     // Load configuration
//!     let config = Config::load_or_create_default().await?;
//!     
//!     // Use the library...
//!     Ok(())
//! }
//! ```

/// Application-wide configuration management.
///
/// Provides configuration loading, validation, and default value handling
/// for all application settings including database paths, logging levels,
/// and integration endpoints.
pub mod app_config;

/// Command Line Interface definitions and handling.
///
/// Contains all CLI command definitions, argument parsing, and command execution logic.
/// Provides a rich set of subcommands for managing tasks, sprints, ADRs, and more.
pub mod cli;

/// Core functionalities: database, data models, error handling, validation.
///
/// The foundation of the application including:
/// - Database operations and connection management
/// - Data models for all entity types
/// - Custom error types and error handling
/// - Data validation and schema enforcement
pub mod core;

/// Business logic services.
///
/// High-level services that combine core functionality to provide complete
/// business operations for each entity type (tasks, sprints, ADRs, etc.).
/// Each service provides CRUD operations and entity-specific logic.
pub mod services;

/// Abstractions and implementations for text embeddings.
///
/// Provides abstractions for generating vector embeddings from text content
/// to enable semantic search and content similarity operations.
pub mod embeddings;

/// Logging setup and utilities.
///
/// Structured logging configuration with support for console and file output,
/// configurable log levels, and integration with the tracing ecosystem.
pub mod logging;

/// Data export functionalities.
///
/// Export services for converting internal data to various external formats
/// including Markdown, JSON, HTML, and other documentation formats.
pub mod export;

/// General utility functions and macros.
///
/// Common utility functions used across the application for string manipulation,
/// date handling, and other cross-cutting concerns.
pub mod utils;

/// Data migration utilities.
///
/// Tools for migrating existing markdown documentation into the llmdocs system,
/// including parsers for various documentation formats and data transformation utilities.
pub mod migration;

// Re-export key components for easier access from main.rs or other binaries/tests
pub use app_config::Config;
pub use cli::Cli;
pub use logging::{Logger, LogConfig};
pub use migration::MarkdownMigrator; // Re-export MarkdownMigrator
pub use embeddings::EmbeddingError;
pub use export::ExportError;
// pub use core::database; // Specific database functions might be re-exported as needed
// pub use services::*; // Or re-export specific services

/// Initializes essential application components.
/// This could include setting up global state, etc., if needed beyond main.rs.
pub async fn initialize_app() -> anyhow::Result<()> {
    // Placeholder for any library-level initialization
    // For now, main.rs handles logging and config loading directly.
    tracing::info!("LLMDocs library initialized.");
    Ok(())
}

// Example of a function that might be part of the library's public API
pub fn get_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}