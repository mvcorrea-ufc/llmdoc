// llmdoc/src/lib.rs

/// Application-wide configuration.
pub mod app_config;

/// Command Line Interface definitions and handling.
pub mod cli;

/// Core functionalities: database, data models, error handling, validation.
pub mod core;

/// Business logic services.
pub mod services;

/// Abstractions and implementations for text embeddings.
pub mod embeddings;

/// Logging setup and utilities.
pub mod logging;

/// Data export functionalities.
pub mod export;

/// General utility functions and macros.
pub mod utils;

// Re-export key components for easier access from main.rs or other binaries/tests
pub use app_config::Config;
pub use cli::Cli;
// pub use core::database; // Specific database functions might be re-exported as needed
// pub use services::*; // Or re-export specific services
// pub use cli::Cli; // Ensure Cli is re-exported for main.rs // This line was a duplicate

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