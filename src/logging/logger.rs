// llmdoc/src/logging/logger.rs
//! Defines the Logger, LogLevel, LogConfig, and logging macros.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};

// Re-using tracing_appender for file logging capabilities
use tracing_appender::rolling;
use tracing_appender::non_blocking::WorkerGuard;


/// Defines the severity levels for log messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal, // Tracing doesn't have Fatal, maps to Error with specific context
}

impl LogLevel {
    pub fn to_tracing_level(&self) -> Level {
        match self {
            LogLevel::Trace => Level::TRACE,
            LogLevel::Debug => Level::DEBUG,
            LogLevel::Info => Level::INFO,
            LogLevel::Warn => Level::WARN,
            LogLevel::Error | LogLevel::Fatal => Level::ERROR,
        }
    }

    pub fn from_str(level_str: &str) -> Result<Self, String> {
        match level_str.to_lowercase().as_str() {
            "trace" => Ok(LogLevel::Trace),
            "debug" => Ok(LogLevel::Debug),
            "info" => Ok(LogLevel::Info),
            "warn" => Ok(LogLevel::Warn),
            "error" => Ok(LogLevel::Error),
            "fatal" => Ok(LogLevel::Fatal),
            _ => Err(format!("Invalid log level: {}", level_str)),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            LogLevel::Trace => "trace",
            LogLevel::Debug => "debug",
            LogLevel::Info => "info",
            LogLevel::Warn => "warn",
            LogLevel::Error => "error",
            LogLevel::Fatal => "fatal",
        }
    }
}

/// Configuration for the logger.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogConfig {
    pub enabled: bool,
    pub level_console: LogLevel,
    pub level_file: LogLevel,
    pub file_path: PathBuf, // Directory for logs, filename will be constructed
    pub file_name_prefix: String,
    // Add rotation and cleanup config later if needed, e.g., max_files, max_size_mb
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            level_console: LogLevel::Info,
            level_file: LogLevel::Debug,
            file_path: PathBuf::from("~/.llmdocs/logs/"), // Default log directory
            file_name_prefix: "llmdoc.log".to_string(),
        }
    }
}

/// The main Logger struct.
/// It will be responsible for initializing and dispatching log messages.
#[derive(Debug)]
pub struct Logger {
    _guard: Option<WorkerGuard>, // Keep the guard for file logging
}

impl Logger {
    /// Initializes the logging system based on the provided configuration.
    /// This adapts logic from the old `logging.rs::init_from_config`.
    pub fn init(config: &LogConfig) -> Result<Self> {
        if !config.enabled {
            // If logging is disabled, we can still return a Logger instance
            // that perhaps does nothing, or we can skip initialization.
            // For now, let's assume if not enabled, we don't set up tracing.
            tracing::info!("Logging is disabled via configuration.");
            return Ok(Logger { _guard: None });
        }

        let log_dir_str = shellexpand::tilde(config.file_path.to_string_lossy().as_ref())
            .into_owned();
        let log_dir = PathBuf::from(log_dir_str);

        if !log_dir.exists() {
            std::fs::create_dir_all(&log_dir)?;
            // Use tracing::event directly if logger isn't fully up yet, or a temporary simple logger.
            // For now, this info message might go to a default subscriber if one exists, or nowhere.
            // println!("Created log directory: {:?}", log_dir); // Simple fallback
        }

        let console_filter = format!("llmdocs={}", config.level_console.as_str());
        let file_filter = format!("llmdocs={}", config.level_file.as_str());

        // File layer
        let file_appender = rolling::daily(&log_dir, &config.file_name_prefix);
        let (non_blocking_file_writer, guard) = tracing_appender::non_blocking(file_appender);

        let file_layer = fmt::layer()
            .with_writer(non_blocking_file_writer)
            .with_ansi(false) // No ANSI colors in files
            .with_filter(EnvFilter::new(file_filter));

        // Console layer
        let console_layer = fmt::layer()
            .with_writer(std::io::stdout)
            .with_filter(EnvFilter::new(console_filter));

        // Combine layers and initialize
        // It's important that try_init is called only once.
        // Subsequent calls will fail. This needs to be handled if re-init is a feature.
        tracing_subscriber::registry()
            .with(console_layer)
            .with(file_layer)
            .try_init()?;

        tracing::event!(Level::INFO, "Logging initialized. Console: {}, File: {} (in {:?})",
            config.level_console.as_str(),
            config.level_file.as_str(),
            log_dir
        );

        Ok(Logger { _guard: Some(guard) })
    }

    // Placeholder for actual logging methods.
    // These would typically be macros for convenience (log_info!, log_error!, etc.)
    // that call a private method on a Logger instance or use tracing::event directly.
}

// Logging macros - these will use the `tracing` crate's macros directly for now.
// The Logger::init() sets up the global subscriber, so tracing macros will use it.

#[macro_export]
macro_rules! log_trace {
    ($($arg:tt)+) => (tracing::trace!($($arg)+))
}

#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)+) => (tracing::debug!($($arg)+))
}

#[macro_export]
macro_rules! log_info {
    ($($arg:tt)+) => (tracing::info!($($arg)+))
}

#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)+) => (tracing::warn!($($arg)+))
}

#[macro_export]
macro_rules! log_error {
    ($($arg:tt)+) => (tracing::error!($($arg)+))
}

#[macro_export]
macro_rules! log_fatal {
    ($($arg:tt)+) => (tracing::error!(target: "FATAL", $($arg)+)) // Use ERROR level, but mark as FATAL
}

// TODO: Add tests for LogLevel, LogConfig, and Logger initialization.
// The tests from the old logging.rs can be adapted here.
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;
    use std::path::Path;

    fn setup_test_log_config(temp_dir: &Path, file_name_prefix: &str, console_level: LogLevel, file_level: LogLevel) -> LogConfig {
        LogConfig {
            enabled: true,
            level_console: console_level,
            level_file: file_level,
            file_path: temp_dir.to_path_buf(),
            file_name_prefix: file_name_prefix.to_string(),
        }
    }

    #[test]
    fn test_logger_init_basic() -> Result<()> {
        let temp_log_dir = tempdir()?.path().join("logs_basic");
        fs::create_dir_all(&temp_log_dir)?;

        let log_config = setup_test_log_config(&temp_log_dir, "test_basic.log", LogLevel::Info, LogLevel::Debug);

        // Attempt to initialize the logger
        let result = Logger::init(&log_config);

        if let Err(e) = &result {
            let error_string = e.to_string().to_lowercase();
            if !error_string.contains("global subscriber has already been set") &&
               !error_string.contains("global default trace dispatcher has already been set") {
                panic!("Logger::init failed with an unexpected error: {:?}", e);
            }
            // If already initialized by another test, that's acceptable for this check
        } else {
            // If Ok, we should have a logger instance
            assert!(result.is_ok());
            let logger = result.unwrap();
            assert!(logger._guard.is_some(), "Logger guard should be present when initialized successfully");
        }
        
        log_info!("Info message from test_logger_init_basic");
        log_debug!("Debug message from test_logger_init_basic");

        // Check if log directory and a log file were created (name will include date)
        assert!(temp_log_dir.exists(), "Log directory should exist.");
        // Check for *any* file starting with the prefix, as date will be appended.
        let entries = fs::read_dir(&temp_log_dir)?
            .filter_map(Result::ok)
            .filter(|entry| entry.file_name().to_string_lossy().starts_with("test_basic.log"))
            .count();
        assert!(entries > 0, "Log file should have been created in {:?}", temp_log_dir);

        Ok(())
    }

    #[test]
    fn test_logger_init_disabled() -> Result<()> {
        let temp_log_dir = tempdir()?.path().join("logs_disabled");
        // fs::create_dir_all(&temp_log_dir)?; // Not strictly needed if logger is disabled

        let mut log_config = setup_test_log_config(&temp_log_dir, "test_disabled.log", LogLevel::Info, LogLevel::Debug);
        log_config.enabled = false;

        let logger_result = Logger::init(&log_config);
        assert!(logger_result.is_ok());
        let logger = logger_result.unwrap();
        assert!(logger._guard.is_none(), "Logger guard should be None when logging is disabled.");
        
        // Check that no log directory was created by the logger
        // (it might exist if created by tempdir() itself, so check for specific log files)
        let log_file_check_path = temp_log_dir.join("test_disabled.log"); // Exact name won't exist
        assert!(!log_file_check_path.exists(), "Log file should not exist when logging is disabled.");
        
        Ok(())
    }

    #[test]
    fn test_loglevel_from_str() {
        assert_eq!(LogLevel::from_str("trace"), Ok(LogLevel::Trace));
        assert_eq!(LogLevel::from_str("DEBUG"), Ok(LogLevel::Debug));
        assert_eq!(LogLevel::from_str("Info"), Ok(LogLevel::Info));
        assert_eq!(LogLevel::from_str("warn"), Ok(LogLevel::Warn));
        assert_eq!(LogLevel::from_str("error"), Ok(LogLevel::Error));
        assert_eq!(LogLevel::from_str("FATAL"), Ok(LogLevel::Fatal));
        assert!(LogLevel::from_str("invalid").is_err());
    }

    #[test]
    fn test_loglevel_to_tracing_level() {
        assert_eq!(LogLevel::Trace.to_tracing_level(), tracing::Level::TRACE);
        assert_eq!(LogLevel::Debug.to_tracing_level(), tracing::Level::DEBUG);
        assert_eq!(LogLevel::Info.to_tracing_level(), tracing::Level::INFO);
        assert_eq!(LogLevel::Warn.to_tracing_level(), tracing::Level::WARN);
        assert_eq!(LogLevel::Error.to_tracing_level(), tracing::Level::ERROR);
        assert_eq!(LogLevel::Fatal.to_tracing_level(), tracing::Level::ERROR);
    }
}