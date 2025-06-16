// llmdoc/src/cli/commands/sprint_cmds.rs

use clap::Subcommand;
use std::sync::Arc;
use colored::*;
use dialoguer::Input;
use serde_json;
use chrono::{Utc, NaiveDate};
use crate::services::sprint_service::SprintServiceTrait;

use crate::core::errors::{Error, Result};
use crate::core::models::{
    sprint::{Sprint, SprintStatus, Retrospective},
};
use crate::cli::output::{print_sprint_table, print_sprint_report, parse_sprint_status};

#[derive(Subcommand, Debug)]
pub enum SprintCommands {
    /// Create a new sprint
    Create {
        #[arg(long)]
        name: Option<String>,
        
        #[arg(long)]
        start: Option<String>,
        
        #[arg(long)]
        end: Option<String>,
    },
    
    /// Get current sprint
    Current,
    
    /// List all sprints
    List {
        #[arg(long)]
        status: Option<String>,
    },
    
    /// Start a sprint
    Start {
        sprint_id: String,
    },
    
    /// Complete a sprint
    Complete {
        sprint_id: String,
        
        #[arg(long)]
        retrospective: bool,
    },
    
    /// Get sprint report
    Report {
        sprint_id: String,
    },
}

pub async fn execute(
    cmd: SprintCommands,
    service: Arc<crate::services::sprint_service::SprintService>,
) -> Result<()> {
    match cmd {
        SprintCommands::Create { name, start, end } => {
            let sprint_name = if let Some(n) = name {
                n
            } else {
                Input::new().with_prompt("Sprint Name").interact_text()?
            };
            
            let start_date_str = if let Some(s) = start {
                s
            } else {
                Input::new().with_prompt("Start Date (YYYY-MM-DD)").interact_text()?
            };
            let end_date_str = if let Some(e) = end {
                e
            } else {
                Input::new().with_prompt("End Date (YYYY-MM-DD)").interact_text()?
            };
            
            let start_date = NaiveDate::parse_from_str(&start_date_str, "%Y-%m-%d")
                .map_err(|e| Error::InvalidInput(format!("Invalid start date format: {}", e)))?
                .and_hms_opt(0, 0, 0).unwrap_or_else(|| Utc::now().naive_utc().date().and_hms_opt(0,0,0).unwrap())
                .and_utc();
            let end_date = NaiveDate::parse_from_str(&end_date_str, "%Y-%m-%d")
                .map_err(|e| Error::InvalidInput(format!("Invalid end date format: {}", e)))?
                .and_hms_opt(23, 59, 59).unwrap_or_else(|| Utc::now().naive_utc().date().and_hms_opt(23,59,59).unwrap())
                .and_utc();
            
            let new_sprint = Sprint {
                id: format!("sprint-{}", Utc::now().format("%Y%m%d%H%M%S")),
                name: sprint_name,
                description: None,
                start_date,
                end_date,
                goals: Vec::new(),
                tasks: Vec::new(),
                status: SprintStatus::Planning,
                velocity: None,
                capacity: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
                retrospective: None,
            };
            
            let created = service.create_sprint(new_sprint).await?;
            println!("{} Sprint {} created", "✓".green(), created.id.cyan());
            Ok(())
        }
        
        SprintCommands::Current => {
            match service.get_current_sprint().await? {
                Some(sprint) => {
                    println!("{}", serde_json::to_string_pretty(&sprint)?);
                }
                None => {
                    println!("No active sprint found.");
                }
            }
            Ok(())
        }
        
        SprintCommands::List { status } => {
            let sprints = service.list_sprints(status.map(|s| parse_sprint_status(&s).unwrap())).await?;
            print_sprint_table(&sprints);
            Ok(())
        }
        
        SprintCommands::Start { sprint_id } => {
            let started_sprint = service.start_sprint(&sprint_id).await?;
            println!("{} Sprint {} started", "✓".green(), started_sprint.id.cyan());
            Ok(())
        }
        
        SprintCommands::Complete { sprint_id, retrospective } => {
            let retro_data = if retrospective {
                Some(create_retrospective_interactive()?)
            } else {
                None
            };
            let completed_sprint = service.complete_sprint(&sprint_id, retro_data).await?;
            println!("{} Sprint {} completed", "✓".green(), completed_sprint.id.cyan());
            Ok(())
        }
        
        SprintCommands::Report { sprint_id } => {
            let report = service.get_sprint_report(&sprint_id).await?;
            print_sprint_report(&report);
            Ok(())
        }
    }
}

// Removed parse_sprint_status as it's now in output.rs

fn create_retrospective_interactive() -> Result<Retrospective> {
    println!("\n--- Sprint Retrospective ---");
    let what_went_well: Vec<String> = Input::<String>::new()
        .with_prompt("What went well? (comma-separated)")
        .allow_empty(true)
        .interact_text()?
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    let what_could_improve: Vec<String> = Input::<String>::new()
        .with_prompt("What could improve? (comma-separated)")
        .allow_empty(true)
        .interact_text()?
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    let action_items: Vec<String> = Input::<String>::new()
        .with_prompt("Action items? (comma-separated)")
        .allow_empty(true)
        .interact_text()?
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    let notes: String = Input::new()
        .with_prompt("Additional notes (optional)")
        .allow_empty(true)
        .interact_text()?;
    
    Ok(Retrospective {
        what_went_well,
        what_could_improve,
        action_items,
        notes: if notes.is_empty() { None } else { Some(notes) },
    })
}