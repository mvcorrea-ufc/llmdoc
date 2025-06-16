// llmdoc/src/cli/commands/mod.rs

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use crate::app_config::Config;
use crate::core::errors::{self, Result}; // Import the errors module and its Result type
use std::sync::Arc;
use std::str::FromStr; // Import FromStr trait
use crate::core::database::DbConnection;
use crate::embeddings::{EmbeddingProvider, EmbeddingProviderProviderType};
use crate::embeddings::http_provider::HttpEmbeddingProvider;
use crate::services::{
    task_service::TaskService,
    sprint_service::SprintService,
    search_service::{SearchService, SearchServiceTrait},
    export_service::{ExportService, ExportServiceTrait},
    component_service::{ComponentService, ComponentServiceTrait},
    adr_service::{AdrService, AdrServiceTrait},
    user_story_service::{UserStoryService, UserStoryServiceTrait},
};
use crate::cli::output; // Import the output module for ExportFormat

/// A struct to hold all initialized services.
#[derive(Clone)]
pub struct Services {
    pub task_service: Arc<TaskService>,
    pub sprint_service: Arc<SprintService>,
    pub search_service: Arc<SearchService>,
    pub export_service: Arc<ExportService>,
    pub component_service: Arc<ComponentService>,
    pub adr_service: Arc<AdrService>,
    pub user_story_service: Arc<UserStoryService>,
    pub embedding_provider: Option<Arc<dyn EmbeddingProvider>>,
}

impl Services {
    /// Initializes all application services.
    pub async fn new(config: &Config, db_connection: Arc<DbConnection>) -> Result<Self> { // Changed DbConnection to Arc<DbConnection>
        let provider_type = FromStr::from_str(&config.embeddings.provider)?;
        let embedding_provider: Option<Arc<dyn EmbeddingProvider>> = match provider_type {
            EmbeddingProviderProviderType::Http => {
                if let Some(http_config) = &config.embeddings.http_provider {
                    Some(Arc::new(HttpEmbeddingProvider::new(
                        http_config.url.clone(),
                        http_config.api_key.clone(), // Pass api_key
                        http_config.model.clone(),
                    )))
                } else {
                    return Err(errors::Error::ConfigError("HTTP embedding provider selected but no configuration found".to_string()));
                }
            }
            EmbeddingProviderProviderType::Native => {
                // Future: Native provider implementation
                return Err(errors::Error::ConfigError("Native embedding provider not yet implemented".to_string()));
            }
            EmbeddingProviderProviderType::None => None,
        };

        let task_service = Arc::new(TaskService::new(db_connection.clone()));
        let sprint_service = Arc::new(SprintService::new(db_connection.clone()));
        let search_service = Arc::new(SearchService::new(db_connection.clone()));
        let export_service = Arc::new(ExportService::new(db_connection.clone()));
        let component_service = Arc::new(ComponentService::new(db_connection.clone()));
        let adr_service = Arc::new(AdrService::new(db_connection.clone()));
        let user_story_service = Arc::new(UserStoryService::new(db_connection.clone()));

        Ok(Self {
            task_service,
            sprint_service,
            search_service,
            export_service,
            component_service,
            adr_service,
            user_story_service,
            embedding_provider,
        })
    }
}

pub mod task_cmds;
pub mod sprint_cmds;
pub mod search_cmds;
pub mod db_cmds;
pub mod init_cmd;
pub mod export_cmd;
pub mod import_cmd;
pub mod watch_cmd;
pub mod migrate_cmd;

#[derive(Parser, Debug)]
#[command(name = "llmdocs")]
#[command(about = "LLM Documentation System", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    #[arg(short, long, global = true, default_value = ".llmdocs/llmdocs.db")]
    pub database: PathBuf,
    
    #[arg(short, long, global = true)]
    pub config: Option<PathBuf>,
    
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new LLMDocs project
    Init {
        #[arg(long)]
        force: bool,
    },
    
    /// Task management commands
    #[command(subcommand)]
    Task(task_cmds::TaskCommands),
    
    /// Sprint management commands
    #[command(subcommand)]
    Sprint(sprint_cmds::SprintCommands),
    
    /// Search commands
    #[command(subcommand)]
    Search(search_cmds::SearchCommands),
    
    /// Export data
    Export {
        #[arg(short, long, default_value = "./docs_export")]
        output: PathBuf,
        
        #[arg(short, long, value_enum, default_value = "markdown")]
        format: output::ExportFormat,
        
        #[arg(long)]
        include_history: bool,
    },
    
    /// Import data
    Import {
        file: PathBuf,
        
        #[arg(long)]
        merge: bool,
    },
    
    /// Database maintenance
    #[command(subcommand)]
    Db(db_cmds::DbCommands),
    
    /// Watch mode for VSCode integration
    Watch {
        #[arg(long, default_value = "1")]
        interval: u64,
    },
    
    /// Migrate from markdown documentation
    Migrate {
        #[arg(short, long, default_value = "./docs")]
        docs_dir: PathBuf,
        
        #[arg(long)]
        dry_run: bool,
    },
}

pub async fn execute(cli: Cli, config: &Config) -> Result<()> {
    // Initialize database connection
    let db_connection = Arc::new(crate::core::database::DbConnection::new( // Wrap in Arc
        &config.database_url
    ).await?);

    // Run migrations
    db_connection.run_migrations().await?;

    // Initialize all services
    let services = Services::new(config, db_connection.clone()).await?;

    match cli.command {
        Commands::Init { force } => {
            init_cmd::init(config, &cli.database, force).await
        }
        Commands::Task(task_cmd) => {
            task_cmds::execute(task_cmd, services.task_service, config).await
        }
        Commands::Sprint(sprint_cmd) => {
            sprint_cmds::execute(sprint_cmd, services.sprint_service).await
        }
        Commands::Search(search_cmd) => {
            if let Some(_provider) = services.embedding_provider {
                let search_service = Arc::new(SearchService::new(db_connection.clone())); // Re-initialize search service with provider
                search_cmds::execute(search_cmd, search_service).await
            } else {
                Err(errors::Error::ConfigError("Embedding provider not configured".to_string()))
            }
        }
        Commands::Export { output, format, include_history } => {
            export_cmd::export(&services.export_service, output, format, include_history).await
        }
        Commands::Import { file, merge } => {
            import_cmd::import(db_connection.clone(), file, merge).await
        }
        Commands::Db(db_cmd) => {
            db_cmds::execute(db_cmd, db_connection.clone()).await
        }
        Commands::Watch { interval } => {
            watch_cmd::watch(db_connection.clone(), interval).await
        }
        Commands::Migrate { docs_dir, dry_run } => {
            migrate_cmd::migrate(db_connection.clone(), docs_dir, dry_run).await
        }
    }
}