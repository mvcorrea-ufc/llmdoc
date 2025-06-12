// llmdoc/src/embeddings/mod.rs

// Define submodules for different embedding providers
// pub mod http_provider;
// pub mod native_provider; // e.g., for ONNX/Candle based models
// pub mod provider_trait; // Trait for embedding providers

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmbeddingError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    #[error("Failed to parse response: {0}")]
    ParseError(String),
    #[error("Embedding provider not configured or invalid: {0}")]
    ConfigError(String),
    #[error("Native model error: {0}")]
    NativeModelError(String),
    #[error("Input error: {0}")]
    InputError(String),
    #[error("Unknown embedding error: {0}")]
    Unknown(String),
}

pub type Embedding = Vec<f32>; // Define an embedding as a vector of f32

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingRequest {
    pub texts: Vec<String>,
    pub model: Option<String>, // Optional: some providers might use a default model
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingResponse {
    pub embeddings: Vec<Embedding>,
    pub model_used: String, // Information about the model that generated the embeddings
    pub usage: Option<UsageStats>, // Optional: token usage, etc.
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UsageStats {
    pub prompt_tokens: u32,
    pub total_tokens: u32,
}


/// Trait for embedding providers.
#[async_trait]
pub trait EmbeddingProvider: Send + Sync {
    /// Generates embeddings for a list of texts.
    async fn generate_embeddings(&self, texts: Vec<String>) -> Result<Vec<Embedding>, EmbeddingError>;
    
    /// Generates embeddings and returns the full response including metadata.
    async fn generate_embeddings_with_response(&self, request: EmbeddingRequest) -> Result<EmbeddingResponse, EmbeddingError>;

    /// Returns the name of the provider (e.g., "http_openai", "native_gte").
    fn provider_name(&self) -> String;
}


pub fn embeddings_init_message() {
    tracing::debug!("Embeddings module initialized (placeholder).");
}

// Example of how one might select a provider based on config
// This would live in a higher-level service or the main logic.
/*
use crate::app_config::Config;
use crate::embeddings::http_provider::HttpEmbeddingProvider; // Assuming this exists
// use crate::embeddings::native_provider::NativeEmbeddingProvider; // Assuming this exists

pub fn get_embedding_provider(config: &Config) -> Result<Box<dyn EmbeddingProvider>, EmbeddingError> {
    match config.embeddings.provider.to_lowercase().as_str() {
        "http" => {
            if let Some(http_config) = &config.embeddings.http_provider {
                Ok(Box::new(HttpEmbeddingProvider::new(
                    &http_config.url,
                    http_config.api_key.as_deref(),
                    http_config.model.as_deref(),
                )))
            } else {
                Err(EmbeddingError::ConfigError("HTTP provider selected but no configuration found".to_string()))
            }
        }
        // "native" => {
        //     if let Some(native_config) = &config.embeddings.native_provider {
        //         // Ok(Box::new(NativeEmbeddingProvider::new(&native_config.model_path)?))
        //         Err(EmbeddingError::ConfigError("Native provider not yet implemented".to_string()))
        //     } else {
        //         Err(EmbeddingError::ConfigError("Native provider selected but no configuration found".to_string()))
        //     }
        // }
        _ => Err(EmbeddingError::ConfigError(format!("Unsupported embedding provider: {}", config.embeddings.provider))),
    }
}
*/