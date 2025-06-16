// llmdoc/src/core/errors.rs

use std::fmt;
// Embedding and export errors are handled as String types for now

#[derive(Debug)]
pub enum Error {
    DatabaseError(rusqlite::Error),
    DatabaseOperation(String),
    ConfigError(String),
    IoError(std::io::Error),
    JsonError(serde_json::Error),
    InvalidInput(String),
    ValidationError(String),
    NotFound(String),
    OperationFailed(String),
    EmbeddingError(String),
    ExportError(String),
    CliArgumentError(String),
    Unknown(String),
    RegexError(regex::Error),
    Parse(chrono::ParseError),
    Dialoguer(dialoguer::Error),
    ParseIntError(std::num::ParseIntError), // Added new variant
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::DatabaseError(e) => write!(f, "Database error: {}", e),
            Error::DatabaseOperation(msg) => write!(f, "Database operation error: {}", msg),
            Error::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            Error::IoError(e) => write!(f, "I/O error: {}", e),
            Error::JsonError(e) => write!(f, "Serialization/Deserialization error (JSON): {}", e),
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            Error::EmbeddingError(e) => write!(f, "Embedding service error: {}", e),
            Error::ExportError(e) => write!(f, "Export service error: {}", e),
            Error::CliArgumentError(msg) => write!(f, "CLI argument error: {}", msg),
            Error::Unknown(msg) => write!(f, "An unknown error occurred: {}", msg),
            Error::RegexError(e) => write!(f, "Regex error: {}", e),
            Error::Parse(e) => write!(f, "Parse error: {}", e),
            Error::Dialoguer(e) => write!(f, "Dialoguer interaction error: {}", e),
            Error::ParseIntError(e) => write!(f, "Parse integer error: {}", e), // Added ParseIntError
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::DatabaseError(e) => Some(e),
            Error::IoError(e) => Some(e),
            Error::JsonError(e) => Some(e),
            Error::RegexError(e) => Some(e),
            Error::Dialoguer(e) => Some(e),
            Error::Parse(e) => Some(e),
            Error::ParseIntError(e) => Some(e), // Added ParseIntError
            _ => None,
        }
    }
}

impl From<rusqlite::Error> for Error {
    fn from(err: rusqlite::Error) -> Self {
        Error::DatabaseError(err)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::JsonError(err)
    }
}

impl From<regex::Error> for Error {
    fn from(err: regex::Error) -> Self {
        Error::RegexError(err)
    }
}

impl From<chrono::ParseError> for Error {
    fn from(err: chrono::ParseError) -> Self {
        Error::Parse(err)
    }
}

impl From<dialoguer::Error> for Error {
    fn from(err: dialoguer::Error) -> Self {
        Error::Dialoguer(err)
    }
}


impl From<toml::ser::Error> for Error {
    fn from(err: toml::ser::Error) -> Self {
        Error::ConfigError(format!("TOML serialization error: {}", err))
    }
}

impl From<toml::de::Error> for Error {
    fn from(err: toml::de::Error) -> Self {
        Error::ConfigError(format!("TOML deserialization error: {}", err))
    }
}

impl From<std::num::ParseIntError> for Error { // Added From implementation
    fn from(err: std::num::ParseIntError) -> Self {
        Error::ParseIntError(err)
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Error::Unknown(err.to_string())
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub fn errors_init_message() {
    tracing::debug!("Errors module initialized (placeholder).");
}