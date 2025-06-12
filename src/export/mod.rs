// llmdoc/src/export/mod.rs

// pub mod markdown_exporter;
// pub mod json_exporter;
// pub mod exporter_trait; // Trait for different exporters

use serde::Serialize;
use thiserror::Error;
use std::path::Path;
use anyhow::Result;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error (JSON): {0}")]
    Json(#[from] serde_json::Error),
    #[error("Templating error (Handlebars): {0}")]
    Handlebars(#[from] handlebars::RenderError),
    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
    #[error("Data not found for export: {0}")]
    DataNotFound(String),
    #[error("Unknown export error: {0}")]
    Unknown(String),
}

/// Trait for data exporters.
pub trait Exporter {
    /// Exports the given serializable data to the specified path.
    /// The format is implicitly defined by the implementer.
    fn export_data<T: Serialize>(&self, data: &T, output_path: &Path) -> Result<(), ExportError>;

    /// Returns the format name this exporter handles (e.g., "markdown", "json").
    fn format_name(&self) -> &str;
}

pub fn export_init_message() {
    tracing::debug!("Export module initialized (placeholder).");
}

// Example of how one might get an exporter
// This would typically be part of an ExportService or similar.
/*
use crate::export::markdown_exporter::MarkdownExporter; // Assuming this exists
use crate::export::json_exporter::JsonExporter;       // Assuming this exists

pub fn get_exporter(format: &str) -> Result<Box<dyn Exporter>, ExportError> {
    match format.to_lowercase().as_str() {
        "markdown" | "md" => Ok(Box::new(MarkdownExporter::new())), // Assuming a simple constructor
        "json" => Ok(Box::new(JsonExporter::new())),
        _ => Err(ExportError::UnsupportedFormat(format.to_string())),
    }
}
*/