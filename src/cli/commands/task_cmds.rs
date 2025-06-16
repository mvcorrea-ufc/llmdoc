// llmdoc/src/cli/commands/task_cmds.rs

use clap::Subcommand;
use std::sync::Arc;
use colored::*;
use dialoguer::{Confirm, Input, Select};
use serde_json;
use crate::services::task_service::TaskServiceTrait; // Add this line

use crate::app_config::Config;
use crate::core::errors::{self, Result}; // Import the errors module and its Result type
use crate::core::models::{
    task::{Task, TaskType},
    validation, // Import the validation module
};
use crate::services::task_service::TaskService;
use crate::cli::output::{print_task_table, print_task, print_task_csv, parse_task_status, OutputFormat};

#[derive(Subcommand, Debug)]
pub enum TaskCommands {
    /// Create a new task
    Add {
        /// Task JSON or interactive mode if not provided
        json: Option<String>,
    },
    
    /// Get task details
    Get {
        task_id: String,
        
        #[arg(long)]
        history: bool,
    },
    
    /// List tasks
    List {
        #[arg(short, long)]
        status: Option<String>,
        
        #[arg(short = 'p', long)]
        sprint: Option<String>,
        
        #[arg(short, long)]
        assignee: Option<String>,
        
        #[arg(long)]
        format: Option<OutputFormat>,
    },
    
    /// Update task
    Update {
        task_id: String,
        
        #[arg(long)]
        status: Option<String>,
        
        #[arg(long)]
        assignee: Option<String>,
        
        #[arg(long)]
        sprint: Option<String>,
        
        #[arg(long)]
        points: Option<u8>,
    },
    
    /// Delete task (soft delete)
    Delete {
        task_id: String,
        
        #[arg(long)]
        force: bool,
    },
    
    /// Bulk operations
    Bulk {
        #[command(subcommand)]
        action: BulkAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum BulkAction {
    /// Update status for multiple tasks
    Status {
        task_ids: Vec<String>,
        status: String,
    },
    
    /// Assign tasks to sprint
    Sprint {
        task_ids: Vec<String>,
        sprint_id: String,
    },
}

pub async fn execute(
    cmd: TaskCommands,
    service: Arc<TaskService>,
    config: &Config, // Keep config for print_task
) -> Result<()> {
    match cmd {
        TaskCommands::Add { json } => {
            let task = if let Some(json) = json {
                serde_json::from_str(&json)?
            } else {
                // Interactive mode
                create_task_interactive()?
            };
            
            let created = service.create_task(task).await?;
            println!("{} Task {} created", "✓".green(), created.id.cyan());
            Ok(())
        }
        
        TaskCommands::Get { task_id, history: _history } => {
            match service.get_task_by_id(task_id.clone()).await? {
                Some(task) => {
                    print_task(&task, config);
                    // History is not yet implemented in TaskService
                    // if history {
                    //     let history = service.get_task_history(&task_id).await?;
                    //     print_task_history(&history);
                    // }
                }
                None => {
                    eprintln!("{} Task {} not found", "✗".red(), task_id);
                }
            }
            Ok(())
        }
        
        TaskCommands::List { status: _status, sprint: _sprint, assignee: _assignee, format } => {
            // TaskQuery is not used directly here, list_tasks is simplified
            let tasks = service.list_tasks().await?;
            
            match format.unwrap_or(OutputFormat::Table) {
                OutputFormat::Table => print_task_table(&tasks),
                OutputFormat::Json => println!("{}", serde_json::to_string_pretty(&tasks)?),
                OutputFormat::Csv => print_task_csv(&tasks)?,
            }
            
            Ok(())
        }
        
        TaskCommands::Update { task_id, status, assignee, sprint, points } => {
            let mut task = service.get_task_by_id(task_id.clone()).await?
                .ok_or_else(|| errors::Error::NotFound(format!("Task {} not found", task_id)))?;
            
            if let Some(s) = status {
                task.status = parse_task_status(&s)?;
            }
            if let Some(a) = assignee {
                task.assignee = Some(a);
            }
            if let Some(s) = sprint {
                task.sprint_id = Some(s);
            }
            if let Some(p) = points {
                task.story_points = Some(p);
            }
            
            let updated = service.update_task(task).await?;
            println!("{} Task {} updated", "✓".green(), updated.id.cyan());
            Ok(())
        }
        
        TaskCommands::Delete { task_id, force } => {
            if force || Confirm::new()
                .with_prompt(format!("Are you sure you want to delete task {}?", task_id))
                .default(false)
                .interact()?
            {
                service.delete_task(task_id.clone()).await?;
                println!("{} Task {} deleted", "✓".green(), task_id.cyan());
            } else {
                println!("Deletion cancelled.");
            }
            Ok(())
        }
        
        TaskCommands::Bulk { action } => {
            match action {
                BulkAction::Status { task_ids: _task_ids, status: _status } => {
                    // Bulk update status not yet implemented in service
                    // let parsed_status = parse_task_status(&status)?;
                    // let updated_count = service.bulk_update_status(&task_ids, parsed_status, None).await?.len();
                    // println!("{} Updated status for {} tasks", "✓".green(), updated_count);
                    println!("{} Bulk status update not yet implemented.", "⚠".yellow());
                    Ok(())
                }
                BulkAction::Sprint { task_ids: _task_ids, sprint_id: _sprint_id } => {
                    // Bulk add to sprint not yet implemented in service
                    // let updated_count = service.bulk_add_to_sprint(&task_ids, &sprint_id, None).await?.len();
                    // println!("{} Added {} tasks to sprint {}", "✓".green(), updated_count, sprint_id.cyan());
                    println!("{} Bulk sprint assignment not yet implemented.", "⚠".yellow());
                    Ok(())
                }
            }
        }
    }
}

fn create_task_interactive() -> Result<Task> {
    let id: String = Input::new()
        .with_prompt("Task ID (e.g., TASK-001)")
        .validate_with(|input: &String| -> std::result::Result<(), &str> { // Use std::result::Result
            if validation::is_valid_task_id(input) {
                Ok(())
            } else {
                Err("Invalid task ID format (use TASK-XXX)")
            }
        })
        .interact_text()?;
    
    let title: String = Input::new()
        .with_prompt("Title")
        .interact_text()?;
    
    let description: String = Input::new()
        .with_prompt("Description (optional)")
        .allow_empty(true)
        .interact_text()?;
    
    let task_types = vec!["feature", "bug", "task", "epic", "story", "spike"];
    let task_type_idx = Select::new()
        .with_prompt("Type")
        .items(&task_types)
        .default(0)
        .interact()?;
    
    let task_type = match task_types[task_type_idx] {
        "feature" => TaskType::Feature,
        "bug" => TaskType::Bug,
        "task" => TaskType::Task,
        "epic" => TaskType::Epic,
        "story" => TaskType::Story,
        "spike" => TaskType::Spike,
        _ => TaskType::Task,
    };
    
    let mut task = Task::new(id, title, task_type);
    
    if !description.is_empty() {
        task.description = Some(description);
    }
    
    Ok(task)
}