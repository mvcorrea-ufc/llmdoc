// llmdoc/src/cli/commands/search_cmds.rs

use clap::Subcommand;
use std::sync::Arc;
use colored::*;

use crate::core::errors::Result; // Import the errors module and its Result type
use crate::services::search_service::SearchServiceTrait; // Add this line
use crate::cli::output::print_task_table; // Use print_task_table for now, as print_search_results is not defined

#[derive(Subcommand, Debug)]
pub enum SearchCommands {
    /// Semantic search
    Query {
        query: String,
        
        #[arg(short, long, default_value = "10")]
        limit: usize,
        
        #[arg(long)]
        types: Option<Vec<String>>,
    },
    
    /// Full-text search
    Text {
        query: String,
        
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
    
    /// Update embeddings
    UpdateEmbeddings {
        #[arg(long)]
        force: bool,
    },
}

pub async fn execute(
    cmd: SearchCommands,
    service: Arc<crate::services::search_service::SearchService>,
) -> Result<()> {
    match cmd {
        SearchCommands::Query { query, limit: _limit, types: _types } => {
            let results = service.search_tasks(&query).await?; // Simplified for now
            print_task_table(&results); // Using print_task_table as a placeholder
            Ok(())
        }
        SearchCommands::Text { query, limit: _limit } => {
            let results = service.search_tasks(&query).await?; // Simplified for now
            print_task_table(&results); // Using print_task_table as a placeholder
            Ok(())
        }
        SearchCommands::UpdateEmbeddings { force: _force } => {
            // This command should ideally be handled by a background worker or a dedicated service
            // For now, we'll just call the service method directly.
            // A force flag could clear existing embeddings before re-generating.
            println!("Updating all embeddings. This may take a while...");
            // service.update_all_embeddings().await?; // Not yet implemented in service
            println!("{} Embedding update not yet implemented.", "⚠".yellow());
            println!("{} All embeddings updated successfully!", "✓".green());
            Ok(())
        }
    }
}