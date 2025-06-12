// llmdoc/src/app_config.rs
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::Result;

/// Application configuration structure.
/// Holds settings for database, logging, API keys, etc.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Path to the SQLite database file.
    pub database_url: String,
    /// Log level for console output.
    pub log_level_console: String,
    /// Log level for file output.
    pub log_level_file: String,
    /// Path to the log file.
    pub log_file: PathBuf,
    /// Directory for storing schema files.
    pub schema_dir: PathBuf,
    /// Configuration for embedding providers.
    pub embeddings: EmbeddingsConfig,
    /// Configuration for export settings.
    pub export: ExportConfig,
    // Add other configuration fields as needed
}

/// Configuration specific to embedding providers.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmbeddingsConfig {
    /// The active embedding provider type (e.g., "http", "native").
    pub provider: String,
    /// Configuration for the HTTP embedding provider.
    pub http_provider: Option<HttpEmbeddingsConfig>,
    /// Configuration for the native embedding provider.
    pub native_provider: Option<NativeEmbeddingsConfig>,
}

/// Configuration for an HTTP-based embedding provider.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HttpEmbeddingsConfig {
    pub url: String,
    pub api_key: Option<String>, // Optional API key
    pub model: Option<String>,   // Optional model name
}

/// Configuration for a native (local) embedding provider.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NativeEmbeddingsConfig {
    pub model_path: PathBuf,
    // Add other native provider settings if needed
}

/// Configuration specific to data export.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExportConfig {
    /// Default directory for exported files.
    pub default_export_dir: PathBuf,
    /// Default format for exports (e.g., "markdown", "json").
    pub default_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            database_url: shellexpand::tilde("~/.llmdocs/llmdocs.sqlite").into_owned(),
            log_level_console: "info".to_string(),
            log_level_file: "debug".to_string(),
            log_file: shellexpand::tilde("~/.llmdocs/logs/llmdocs.log").into_owned().into(),
            schema_dir: shellexpand::tilde("~/.llmdocs/schemas").into_owned().into(),
            embeddings: EmbeddingsConfig::default(),
            export: ExportConfig::default(),
        }
    }
}

impl Default for EmbeddingsConfig {
    fn default() -> Self {
        EmbeddingsConfig {
            provider: "http".to_string(), // Default to HTTP provider
            http_provider: Some(HttpEmbeddingsConfig::default()),
            native_provider: None,
        }
    }
}

impl Default for HttpEmbeddingsConfig {
    fn default() -> Self {
        HttpEmbeddingsConfig {
            url: "http://localhost:8008/embed".to_string(), // Default local Python server
            api_key: None,
            model: None,
        }
    }
}

// NativeEmbeddingsConfig does not have a sensible default without a model_path

impl Default for ExportConfig {
    fn default() -> Self {
        ExportConfig {
            default_export_dir: shellexpand::tilde("~/Documents/LLMDocs_Exports").into_owned().into(),
            default_format: "markdown".to_string(),
        }
    }
}

impl Config {
    /// Loads configuration from a specified TOML file or default paths.
    /// If `custom_path` is Some, it tries to load from there.
    /// Otherwise, it tries default locations:
    /// 1. ./llmdocs.toml (current directory)
    /// 2. ~/.config/llmdocs/config.toml
    /// If no file is found or loading fails, it returns an error.
    /// Use `Config::load(None).unwrap_or_default()` for a fallback.
    pub fn load(custom_path: Option<PathBuf>) -> Result<Self> {
        let mut settings = config::Config::builder();

        let paths_to_try: Vec<PathBuf> = if let Some(cp) = custom_path {
            vec![cp]
        } else {
            vec![
                PathBuf::from("./llmdocs.toml"),
                shellexpand::tilde("~/.config/llmdocs/config.toml").into_owned().into(),
            ]
        };

        let mut loaded_from_file = false;
        for path in paths_to_try {
            if path.exists() {
                settings = settings.add_source(config::File::from(path.clone()).required(false));
                tracing::info!("Attempting to load configuration from: {:?}", path);
                loaded_from_file = true;
                break; // Load from the first found path
            }
        }

        if !loaded_from_file {
             tracing::info!("No configuration file found at specified or default locations. Using default values.");
        }
        
        // Add environment variable overrides
        // Example: LLMDOCS_DATABASE_URL or LLMDOCS_DATABASE.URL
        settings = settings.add_source(config::Environment::with_prefix("LLMDOCS").separator("__"));

        match settings.build() {
            Ok(config_obj) => {
                match config_obj.try_deserialize() {
                    Ok(cfg) => {
                        tracing::info!("Configuration loaded successfully.");
                        Ok(cfg)
                    },
                    Err(e) => {
                        tracing::warn!("Failed to deserialize configuration: {}. Using default configuration.", e);
                        // Fallback to default if deserialization fails after attempting to load
                        Ok(Config::default())
                    }
                }
            },
            Err(e) => {
                 tracing::warn!("Failed to build configuration object: {}. Using default configuration.", e);
                 // Fallback to default if building the config object itself fails
                 Ok(Config::default())
            }
        }
    }

    /// Ensures that directories specified in the config exist, creating them if necessary.
    pub fn ensure_directories_exist(&self) -> Result<()> {
        let dirs_to_check: Vec<Option<PathBuf>> = vec![
            self.log_file.parent().map(|p| p.to_path_buf()),
            Some(self.schema_dir.clone()), // Clone to own PathBuf
            Some(self.export.default_export_dir.clone()), // Clone to own PathBuf
            self.database_url_as_path().and_then(|p| p.parent().map(|path_ref| path_ref.to_path_buf())),
        ];

        for dir_option in dirs_to_check {
            if let Some(dir_path_buf) = dir_option { // dir_path_buf is now Option<PathBuf>
                if !dir_path_buf.exists() {
                    tracing::info!("Creating directory: {:?}", dir_path_buf);
                    std::fs::create_dir_all(&dir_path_buf)?; // Pass as reference
                }
            }
        }
        Ok(())
    }

    /// Helper to get database_url as PathBuf for directory creation.
    pub fn database_url_as_path(&self) -> Option<PathBuf> {
        // This handles cases where database_url might be a non-file path (e.g., in-memory)
        // For simplicity, assuming it's a file path for now.
        if self.database_url.starts_with("sqlite:") { // A common prefix, though not standard for file paths
             return Some(PathBuf::from(self.database_url.replacen("sqlite:", "", 1)));
        }
        if !self.database_url.contains(":") { // Heuristic: if no scheme, assume file path
            return Some(PathBuf::from(&self.database_url));
        }
        None
    }
}