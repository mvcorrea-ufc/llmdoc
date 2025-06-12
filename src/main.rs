// llmdoc/src/main.rs
use anyhow::Result;
use clap::Parser;
use llmdocs::app_config::Config;
use llmdocs::cli::Cli;
use llmdocs::logging;
use tracing_appender::non_blocking::WorkerGuard; // For holding the log guard

// Store the guard globally or pass it around. For a simple CLI app,
// leaking it or holding it in main is common.
// Making it static ensures it lives for the duration of the program.
// However, returning it from init_from_config and holding it in main is cleaner.
static mut LOG_GUARD: Option<WorkerGuard> = None;


#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration first, as logging setup might depend on it.
    // The custom_path for config can be parsed from initial CLI args if needed,
    // or use environment variables. For now, `None` uses default paths.
    let config = Config::load(None)?; // Use ? to propagate error if loading fails fundamentally
                                       // Config::load now returns default on deserialize error, logging a warning.

    // Ensure directories specified in config exist (e.g., for logs, database)
    // This should be done after loading config and before initializing components that use these dirs.
    if let Err(e) = config.ensure_directories_exist() {
        // Use a basic print here as tracing might not be set up or might fail if log dir creation failed.
        eprintln!("Failed to create necessary application directories: {}. Exiting.", e);
        // Optionally, try a very basic logger setup here for this specific error if possible,
        // or just exit.
        return Err(e);
    }

    // Initialize logging using the loaded configuration.
    // The guard must be kept alive for file logging to work.
    let _guard = match logging::init_from_config(&config) {
        Ok(g) => g,
        Err(e) => {
            eprintln!("Failed to initialize logging: {}. Some logs may be lost. Continuing with basic stdout logging if possible.", e);
            // Fallback or exit if logging is critical
            // For now, we'll let it proceed, but tracing might not work as expected.
            // A simple panic might be appropriate if logging is absolutely essential from the start.
            // Or, setup a very basic stdout logger here.
            // Let's try to set up a minimal stdout logger if init_from_config fails.
            logging::init_logging_stdout_and_file("llmdocs_fallback.log", tracing::Level::INFO, tracing::Level::DEBUG)?;
            // This fallback guard is not stored, so file logging might be brief.
            // The primary guard is what we want to keep.
            // This path indicates a problem, so exiting might be better.
            return Err(e.into()); // Convert logging error to anyhow::Error and exit
        }
    };
    // Store the guard to keep it alive.
    // A common way for applications is to Box::leak(guard) or store it in a static variable.
    // Or, if main were a struct, store it as a field.
    // For this simple main, we can assign it to `_guard` which lives for the scope of main.
    // If `init_from_config` returned the guard, we'd assign `let logging_guard = ...;`
    // The `_guard` binding ensures it's not dropped immediately.


    tracing::info!("LLMDocs application starting...");
    tracing::debug!("Configuration loaded: {:?}", config);

    // Parse CLI arguments
    let cli_args = Cli::parse();
    tracing::debug!("CLI arguments parsed: {:?}", cli_args);

    // TODO: Initialize database connection pool based on config
    // let db_pool = llmdocs::core::database::init_pool(&config.database_url).await?;

    // Execute the command based on parsed arguments
    llmdocs::cli::process_cli_command(cli_args, &config).await?;
    
    tracing::info!("LLMDocs application finished.");
    Ok(())
}