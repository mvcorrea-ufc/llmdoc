// llmdoc/src/main.rs
use anyhow::Result;
use clap::Parser;
use llmdocs::app_config::Config;
use llmdocs::cli::Cli;
use llmdocs::logging::{Logger, LogConfig, LogLevel}; // Import new logging components
use llmdocs::log_info; // Import the log_info macro


#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration first, as logging setup might depend on it.
    let config = Config::load(None)?;

    // Ensure directories specified in config exist
    if let Err(e) = config.ensure_directories_exist() {
        eprintln!("Failed to create necessary application directories: {}. Exiting.", e);
        return Err(e);
    }

    // Prepare LogConfig from app_config::Config
    let log_level_console = LogLevel::from_str(&config.log_level_console)
        .unwrap_or(LogLevel::Info); // Default to Info on parse error
    let log_level_file = LogLevel::from_str(&config.log_level_file)
        .unwrap_or(LogLevel::Debug); // Default to Debug on parse error

    let log_file_path = config.log_file.parent().ok_or_else(|| {
        anyhow::anyhow!("Log file path {:?} does not have a parent directory.", config.log_file)
    })?.to_path_buf();

    let log_file_name_prefix = config.log_file.file_name().ok_or_else(|| {
        anyhow::anyhow!("Log file path {:?} does not have a file name.", config.log_file)
    })?.to_string_lossy().into_owned();

    let app_log_config = LogConfig {
        enabled: true, // Assuming logging is always enabled if this section is reached.
                       // Could be tied to a field in app_config::Config if needed.
        level_console: log_level_console,
        level_file: log_level_file,
        file_path: log_file_path,
        file_name_prefix: log_file_name_prefix,
    };

    // Initialize logging using the new Logger
    // The Logger instance itself holds the guard, so it must be kept alive.
    let _logger = match Logger::init(&app_log_config) {
        Ok(logger_instance) => logger_instance,
        Err(e) => {
            eprintln!("Failed to initialize logging: {}. Some logs may be lost. Exiting.", e);
            // If logging is critical, exiting is safer.
            return Err(e.into());
        }
    };

    // Logging macros (log_info!, log_debug!, etc.) are now globally available
    // if exported correctly from the llmdocs::logging module.
    // They use the globally initialized `tracing` subscriber.
    log_info!("LLMDocs application starting...");
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