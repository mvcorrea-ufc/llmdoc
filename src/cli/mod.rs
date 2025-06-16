// llmdoc/src/cli/mod.rs

use crate::app_config::Config;
use crate::core::errors::{Error, Result}; // Import Error and Result

pub mod commands;
pub mod output;

pub use commands::{Cli, Commands};

pub async fn process_cli_command(cli: Cli, config: &Config) -> Result<(), Error> { // Change return type to include Error
    commands::execute(cli, config).await
}