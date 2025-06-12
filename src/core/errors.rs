// llmdoc/src/core/errors.rs

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization/Deserialization error (JSON): {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("Validation error: {0}")]
    ValidationError(String), // Or a more specific validation error type

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Embedding service error: {0}")]
    EmbeddingError(#[from] crate::embeddings::EmbeddingError), // Assuming EmbeddingError is in crate::embeddings

    #[error("Export service error: {0}")]
    ExportError(#[from] crate::export::ExportError), // Assuming ExportError is in crate::export

    #[error("CLI argument error: {0}")]
    CliArgumentError(String),

    #[error("An unknown error occurred: {0}")]
    Unknown(String),

    // Add more specific error types as needed
}

// Helper for converting other errors to AppError if needed,
// though `#[from]` handles many cases.
// impl From<some_other_crate::Error> for AppError {
//     fn from(err: some_other_crate::Error) -> Self {
//         AppError::Unknown(err.to_string())
//     }
// }

pub fn errors_init_message() {
    tracing::debug!("Errors module initialized (placeholder).");
}