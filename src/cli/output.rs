// llmdoc/src/cli/output.rs
use anyhow::Result;
use crate::core::models::{
    task::{Task, TaskStatus, TaskType},
    sprint::{Sprint, SprintStatus},
};
use chrono::{DateTime, Local, Utc};
use colored::*;
use comfy_table::{presets::*, Cell, Color, ContentArrangement, Table};

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum ExportFormat {
    Markdown,
    Json,
    Html,
}

/// Print a formatted task table
pub fn print_task_table(tasks: &[Task]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID").fg(Color::Blue),
            Cell::new("Title"),
            Cell::new("Status"),
            Cell::new("Type"),
            Cell::new("Sprint"),
            Cell::new("Assignee"),
            Cell::new("Points"),
            Cell::new("Updated"),
        ]);
    
    for task in tasks {
        let status_cell = match task.status {
            TaskStatus::Todo => Cell::new("TODO").fg(Color::White),
            TaskStatus::InProgress => Cell::new("IN PROGRESS").fg(Color::Yellow),
            TaskStatus::Done => Cell::new("DONE").fg(Color::Green),
            TaskStatus::Blocked => Cell::new("BLOCKED").fg(Color::Red),
            TaskStatus::Cancelled => Cell::new("CANCELLED").fg(Color::DarkGrey),
        };
        
        let type_cell = match task.task_type {
            TaskType::Bug => Cell::new("🐛 Bug").fg(Color::Red),
            TaskType::Feature => Cell::new("✨ Feature").fg(Color::Green),
            TaskType::Task => Cell::new("📋 Task"),
            TaskType::Epic => Cell::new("🏔️ Epic").fg(Color::Magenta),
            TaskType::Story => Cell::new("📖 Story").fg(Color::Blue),
            TaskType::Spike => Cell::new("🔬 Spike").fg(Color::Yellow),
        };
        
        table.add_row(vec![
            Cell::new(&task.id).fg(Color::Cyan),
            Cell::new(&task.title),
            status_cell,
            type_cell,
            Cell::new(task.sprint_id.as_deref().unwrap_or("-")),
            Cell::new(task.assignee.as_deref().unwrap_or("-")),
            Cell::new(task.story_points.map_or("-".to_string(), |p| p.to_string())),
            Cell::new(format_datetime(&task.updated_at)),
        ]);
    }
    
    println!("{table}");
    println!("\n{} {} tasks", "Total:".dimmed(), tasks.len());
}

/// Print a single task with details
pub fn print_task(task: &Task, _config: &crate::app_config::Config) {
    println!("{}", format!("Task {}", task.id).cyan().bold());
    println!("{}", "═".repeat(50));
    
    println!("{}: {}", "Title".bold(), task.title);
    
    if let Some(desc) = &task.description {
        println!("{}: {}", "Description".bold(), desc);
    }
    
    let status_str = match task.status {
        TaskStatus::Todo => "TODO".white(),
        TaskStatus::InProgress => "IN PROGRESS".yellow(),
        TaskStatus::Done => "DONE".green(),
        TaskStatus::Blocked => "BLOCKED".red(),
        TaskStatus::Cancelled => "CANCELLED".dimmed(),
    };
    println!("{}: {}", "Status".bold(), status_str);
    
    println!("{}: {}", "Type".bold(), format!("{:?}", task.task_type));
    println!("{}: {}", "Priority".bold(), format!("{:?}", task.priority));
    
    if let Some(sprint) = &task.sprint_id {
        println!("{}: {}", "Sprint".bold(), sprint);
    }
    
    if let Some(assignee) = &task.assignee {
        println!("{}: {}", "Assignee".bold(), assignee);
    }
    
    if let Some(points) = task.story_points {
        println!("{}: {}", "Story Points".bold(), points);
    }
    
    if !task.labels.is_empty() {
        println!("{}: {}", "Labels".bold(), task.labels.join(", "));
    }
    
    if !task.dependencies.is_empty() {
        println!("{}: {}", "Dependencies".bold(), task.dependencies.join(", "));
    }
    
    println!("\n{}", "Timestamps".dimmed());
    println!("  Created: {} {}", 
        format_datetime(&task.created_at),
        task.created_by.as_deref().unwrap_or("").dimmed()
    );
    println!("  Updated: {} {}",
        format_datetime(&task.updated_at),
        task.updated_by.as_deref().unwrap_or("").dimmed()
    );
    
    if let Some(completed) = &task.completed_at {
        println!("  Completed: {}", format_datetime(completed).green());
    }
}

/// Print task history
pub fn print_task_history(history: &[serde_json::Value]) {
    println!("\n{}", "History".cyan().bold());
    println!("{}", "─".repeat(50));
    
    for (idx, entry) in history.iter().enumerate() {
        if let Some(meta) = entry.get("_meta") {
            let version = meta["version"].as_i64().unwrap_or(0);
            let changed_at = meta["changed_at"].as_str().unwrap_or("");
            let change_type = meta["change_type"].as_str().unwrap_or("");
            let changed_by = meta["changed_by"].as_str().unwrap_or("unknown");
            
            println!(
                "{} {} - {} by {}",
                format!("v{}", version).dimmed(),
                changed_at.dimmed(),
                change_type.yellow(),
                changed_by.cyan()
            );
            
            // Show what changed (simplified)
            if idx < history.len() - 1 {
                // Compare with previous version
                // This is simplified - in production, use a proper diff library
                println!("  Changes made in this version");
            }
        }
    }
}

/// Format datetime for display
fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.with_timezone(&Local).format("%Y-%m-%d %H:%M").to_string()
}

/// Print tasks as CSV
pub fn print_task_csv(tasks: &[Task]) -> Result<()> {
    let mut wtr = csv::Writer::from_writer(std::io::stdout());
    
    // Write header
    wtr.write_record(&[
        "ID", "Title", "Description", "Status", "Type", "Priority",
        "Sprint", "Assignee", "Points", "Created", "Updated"
    ])?;
    
    // Write tasks
    for task in tasks {
        wtr.write_record(&[
            &task.id,
            &task.title,
            task.description.as_deref().unwrap_or(""),
            &format!("{:?}", task.status),
            &format!("{:?}", task.task_type),
            &format!("{:?}", task.priority),
            task.sprint_id.as_deref().unwrap_or(""),
            task.assignee.as_deref().unwrap_or(""),
            &task.story_points.map_or(String::new(), |p| p.to_string()),
            &task.created_at.to_rfc3339(),
            &task.updated_at.to_rfc3339(),
        ])?;
    }
    
    wtr.flush()?;
    Ok(())
}

/// Parse task status from string
pub fn parse_task_status(s: &str) -> Result<TaskStatus> {
    match s.to_lowercase().as_str() {
        "todo" | "to-do" | "to_do" => Ok(TaskStatus::Todo),
        "in_progress" | "in-progress" | "inprogress" | "doing" => Ok(TaskStatus::InProgress),
        "done" | "completed" | "complete" => Ok(TaskStatus::Done),
        "blocked" => Ok(TaskStatus::Blocked),
        "cancelled" | "canceled" => Ok(TaskStatus::Cancelled),
        _ => Err(crate::core::errors::Error::InvalidInput(format!("Invalid status: {}", s)).into()),
    }
}

/// Print a formatted sprint table
pub fn print_sprint_table(sprints: &[Sprint]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("ID").fg(Color::Blue),
            Cell::new("Name"),
            Cell::new("Status"),
            Cell::new("Start Date"),
            Cell::new("End Date"),
            Cell::new("Updated"),
        ]);

    for sprint in sprints {
        let status_cell = match sprint.status {
            SprintStatus::Planning => Cell::new("PLANNING").fg(Color::White),
            SprintStatus::Active => Cell::new("ACTIVE").fg(Color::Green),
            SprintStatus::Completed => Cell::new("COMPLETED").fg(Color::Blue),
            SprintStatus::Cancelled => Cell::new("CANCELLED").fg(Color::Red),
        };

        table.add_row(vec![
            Cell::new(&sprint.id).fg(Color::Cyan),
            Cell::new(&sprint.name),
            status_cell,
            Cell::new(format_datetime(&sprint.start_date)),
            Cell::new(format_datetime(&sprint.end_date)),
            Cell::new(format_datetime(&sprint.updated_at)),
        ]);
    }

    println!("{table}");
    println!("\n{} {} sprints", "Total:".dimmed(), sprints.len());
}

/// Print a sprint report
pub fn print_sprint_report(report: &crate::services::sprint_service::SprintReport) {
    println!("{}", format!("Sprint Report: {}", report.sprint_name).cyan().bold());
    println!("{}", "═".repeat(50));

    println!("{}: {}", "Sprint ID".bold(), report.sprint_id);
    println!("{}: {}", "Total Tasks".bold(), report.total_tasks);
    println!("{}: {}", "Completed Tasks".bold(), report.completed_tasks);
    println!("{}: {}", "Remaining Tasks".bold(), report.remaining_tasks);
    println!("{}: {}", "Total Story Points".bold(), report.total_story_points);
    println!("{}: {}", "Completed Story Points".bold(), report.completed_story_points);
    println!("{}: {}", "Remaining Story Points".bold(), report.remaining_story_points);

    if let Some(retro) = &report.retrospective {
        println!("\n{}", "Retrospective".dimmed());
        if !retro.what_went_well.is_empty() {
            println!("  {}: {}", "What Went Well".bold(), retro.what_went_well.join(", "));
        }
        if !retro.what_could_improve.is_empty() {
            println!("  {}: {}", "What Could Improve".bold(), retro.what_could_improve.join(", "));
        }
        if !retro.action_items.is_empty() {
            println!("  {}: {}", "Action Items".bold(), retro.action_items.join(", "));
        }
        if let Some(notes) = &retro.notes {
            println!("  {}: {}", "Notes".bold(), notes);
        }
    }
}

/// Parse sprint status from string
pub fn parse_sprint_status(s: &str) -> Result<SprintStatus> {
    match s.to_lowercase().as_str() {
        "planning" => Ok(SprintStatus::Planning),
        "active" => Ok(SprintStatus::Active),
        "completed" => Ok(SprintStatus::Completed),
        "cancelled" => Ok(SprintStatus::Cancelled),
        _ => Err(crate::core::errors::Error::InvalidInput(format!("Invalid sprint status: {}", s)).into()),
    }
}