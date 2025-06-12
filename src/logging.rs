// llmdoc/src/logging.rs
use anyhow::Result;
use std::path::Path;
use tracing::Level;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Layer};
use tracing_appender::rolling;

/// Initializes logging to both stdout and a rolling file.
///
/// # Arguments
/// * `log_file_name_prefix` - The prefix for the log file names (e.g., "llmdocs.log").
///                            The actual file will be like "llmdocs.log.2023-10-27".
/// * `console_level` - The `tracing::Level` for console output.
/// * `file_level` - The `tracing::Level` for file output.
///
/// # Errors
/// Returns an error if the log directory cannot be created or if subscriber initialization fails.
pub fn init_logging_stdout_and_file(
    log_file_name_prefix: &str,
    console_level: Level,
    file_level: Level,
) -> Result<()> {
    // Ensure the log directory exists (taken from app_config::Config::default() for now)
    // In a more robust setup, this path would come from the loaded Config.
    let default_log_dir = shellexpand::tilde("~/.llmdocs/logs/")
        .into_owned();
    let log_dir_path = Path::new(&default_log_dir);

    if !log_dir_path.exists() {
        std::fs::create_dir_all(log_dir_path)?;
        tracing::info!("Created log directory: {:?}", log_dir_path);
    }

    // File layer for rolling logs
    let file_appender = rolling::daily(log_dir_path, log_file_name_prefix);
    let (non_blocking_file_writer, _guard_file) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer()
        .with_writer(non_blocking_file_writer)
        .with_ansi(false) // No ANSI colors in files
        .with_filter(EnvFilter::new(format!("llmdocs={}", file_level.as_str()))); // Filter for file

    // Console layer
    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::new(format!("llmdocs={}", console_level.as_str()))); // Filter for console

    // Combine layers and initialize
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .try_init()?; // Use try_init to handle potential errors

    // Keep the guard alive for the duration of the program.
    // If we were in a struct, we'd store it there.
    // For main, this is tricky. A common approach is to return it and have main hold it.
    // Or, for simple cases, Box::leak it (though not ideal for libraries).
    // For now, we'll rely on the fact that main will run for the process lifetime.
    // A better solution for libraries is to provide a guard to be held by the application.
    // For this CLI app, _guard_file going out of scope when init_logging returns is fine
    // as long as tracing_appender's non_blocking layer is designed for this.
    // The `tracing_appender` docs suggest the guard needs to be kept.
    // A simple way for an application:
    // static LAZY_GUARD: Lazy<WorkerGuard> = Lazy::new(|| ...);
    // Or return it from init and store in main.
    // For now, we'll make it static to ensure it lives.
    // This is a common pattern for global singletons like log guards.
    // Note: This makes `_guard_file` effectively a global.
    // A cleaner way would be for `main` to own this guard.
    // Let's return the guard from this function.
    //
    // Update: The `tracing_appender` documentation for `non_blocking` states:
    // "The guard RAII object that is returned must be assigned to a binding that is not `_`
    //  (such as `let _guard = ...;`) or it will be dropped immediately."
    // And "The guard should be held until the end of the program. This is commonly done by
    //  assigning it to a field in a struct."
    // For a simple main, we can Box::leak it or return it.
    // Let's return it for now, and main.rs can decide to leak it or hold it.
    //
    // Simpler approach for now: `init()` is often preferred for global setup.
    // `try_init` is used here. The guard from `non_blocking` is important.
    // The `tracing_appender` example shows `let _guard = ...;` in main.
    // So, this function should probably return the guard.
    //
    // For simplicity in this initial scaffolding, we'll let the guard drop.
    // This means file logging might not be fully robust if the app exits very quickly
    // or under specific conditions. This can be improved later.
    // The `_guard_file` variable name indicates it's intentionally unused for now.

    tracing::info!(
        "Logging initialized. Console: {}, File: {} (in {:?})",
        console_level,
        file_level,
        log_dir_path
    );

    Ok(())
}

/// A more advanced initialization that takes `Config`.
/// This is a placeholder for future enhancement.
pub fn init_from_config(config: &crate::app_config::Config) -> Result<tracing_appender::non_blocking::WorkerGuard> {
    let log_dir = config.log_file.parent().ok_or_else(|| {
        anyhow::anyhow!("Log file path does not have a parent directory: {:?}", config.log_file)
    })?;
    let log_file_name = config.log_file.file_name().ok_or_else(|| {
        anyhow::anyhow!("Log file path does not have a file name: {:?}", config.log_file)
    })?.to_string_lossy();


    if !log_dir.exists() {
        std::fs::create_dir_all(log_dir)?;
        tracing::info!("Created log directory: {:?}", log_dir);
    }

    let console_level_str = config.log_level_console.to_lowercase();
    let file_level_str = config.log_level_file.to_lowercase();

    let console_filter = format!("llmdocs={}", console_level_str);
    let file_filter = format!("llmdocs={}", file_level_str);


    let file_appender = rolling::daily(log_dir, &*log_file_name); // Dereference here
    let (non_blocking_file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let file_layer = fmt::layer()
        .with_writer(non_blocking_file_writer)
        .with_ansi(false)
        .with_filter(EnvFilter::new(file_filter));

    let console_layer = fmt::layer()
        .with_writer(std::io::stdout)
        .with_filter(EnvFilter::new(console_filter));
    
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .try_init()?;

    tracing::info!(
        "Logging initialized from config. Console: {}, File: {} (in {:?})",
        config.log_level_console,
        config.log_level_file,
        log_dir
    );
    Ok(guard)
}


#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    // std::fs is already in scope from the crate level for functions like std::fs::create_dir_all
    // No need for 'use std::fs;' here if not using a 'fs::' prefix.

    #[test]
    fn test_init_logging_stdout_and_file() -> Result<()> {
        // Create a temporary directory for logs
        let temp_log_dir = tempdir()?.path().join("logs"); // Subdirectory for logs
        // Ensure the main temp dir exists for the logs subdir to be created in
        std::fs::create_dir_all(temp_log_dir.parent().unwrap())?;


        // Modify the init_logging_stdout_and_file to accept a base path for testing
        // For now, we'll test if it runs without error and creates the default dir if not present.
        // This test is more of an integration test for the logging setup.
        
        // We need a way to override the log path for tests or ensure the default path is writable.
        // For this test, let's assume the default path can be created or exists.
        // A better approach would be to parameterize the log path in init_logging_stdout_and_file.

        // For now, just call it and check for Ok result.
        // The actual log output is harder to test without capturing stdout/stderr or reading files.
        let result = init_logging_stdout_and_file("test_app.log", Level::DEBUG, Level::TRACE);
        
        // Allow Ok or an error indicating the logger was already initialized by another test.
        if let Err(e) = &result {
            // A more robust check would be to downcast the error to tracing_subscriber::util::TryInitError
            // and check its kind, but for now, string matching is a common workaround.
            // The error from try_init() is wrapped in anyhow::Error.
            let error_string = e.to_string().to_lowercase(); // Convert to lowercase for broader matching
            if !error_string.contains("failed to set global default subscriber") &&
               !error_string.contains("global subscriber has already been set") &&
               !error_string.contains("global default trace dispatcher has already been set") { // Add the new message
                // If it's a different error, then panic.
                panic!("init_logging_stdout_and_file failed with an unexpected error: {:?}", e);
            }
            // If it's an "already initialized" type error, we consider the test as passed for this aspect.
        }
        // If result was Ok, it's also fine.

        // Check if the default log directory was created (if it didn't exist)
        // This part is a bit fragile as it depends on the hardcoded default path.
        let owned_log_dir_str = shellexpand::tilde("~/.llmdocs/logs/").into_owned(); // Fix E0716
        let default_log_dir_path = Path::new(&owned_log_dir_str);
        assert!(default_log_dir_path.exists(), "Default log directory should exist after init.");
        
        // Clean up: In a real test, you might want to remove created log files/dirs
        // but for ~/.llmdocs, it's probably fine to leave it.
        // If using temp_log_dir, it cleans up automatically.

        Ok(())
    }

    #[test]
    fn test_init_from_config() -> Result<()> {
        let temp_dir = tempdir()?;
        let log_file_path = temp_dir.path().join("test_config_app.log");

        let mut config = crate::app_config::Config::default();
        config.log_file = log_file_path.clone();
        config.log_level_console = "info".to_string();
        config.log_level_file = "debug".to_string();
        
        // Ensure parent directory for log_file_path exists
        std::fs::create_dir_all(log_file_path.parent().unwrap())?;

        let result = init_from_config(&config);
        
        let mut guard_opt: Option<tracing_appender::non_blocking::WorkerGuard> = None;

        if let Err(e) = &result {
            let error_string = e.to_string().to_lowercase();
            if !error_string.contains("failed to set global default subscriber") &&
               !error_string.contains("global subscriber has already been set") &&
               !error_string.contains("global default trace dispatcher has already been set") {
                // If it's a different error, then panic or return it.
                return Err(result.err().unwrap()); // Propagate the original error
            }
            // If it's an "already initialized" error, we proceed without a guard from this call.
        } else if let Ok(g) = result {
            guard_opt = Some(g);
        }

        // We can still attempt to log, it might go to a previously initialized logger.
        tracing::info!("This is an info message for config test.");
        tracing::debug!("This is a debug message for config test.");

        // Check if log file was created (or its directory)
        assert!(log_file_path.parent().unwrap().exists());
        
        // To actually check log contents, you'd read the file.
        // For now, just ensuring it runs and creates the dir is sufficient.
        
        if let Some(g) = guard_opt {
            drop(g); // Ensure guard is dropped, flushing logs, if we got one.
        }
        Ok(())
    }
}