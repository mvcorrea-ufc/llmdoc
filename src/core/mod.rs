// llmdoc/src/core/mod.rs

pub mod database;
pub mod models;
pub mod errors;
// pub mod migrations; // If migrations are handled via Rust code

// Re-export key components from submodules if desired
// pub use database::DatabaseConnection;
// pub use models::Task;
// pub use errors::AppError;

pub fn core_init_message() {
    tracing::debug!("Core module initialized (placeholder).");
}