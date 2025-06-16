// llmdoc/src/migration/tasks.rs

use crate::{
    core::{
        models::{Task, TaskType},
        errors::Result,
    },
    services::{
        task_service::TaskServiceTrait,
    },
};
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use tracing::{warn};

use super::MarkdownMigrator; // Import from the parent module

impl MarkdownMigrator {
    pub(super) async fn migrate_tasks(&mut self) -> Result<()> {
        let tasks_file = self.docs_dir.join("agile/tasks.md");
        if !tasks_file.exists() {
            warn!("tasks.md not found at {:?}", tasks_file);
            return Ok(());
        }

        println!("📋 Migrating tasks...");
        let content = std::fs::read_to_string(&tasks_file)?;

        // Parse task sections with improved regex
        let task_regex = Regex::new(
            r"(?m)^###?\s+(\w+-\d+):\s*(.+?)$\n((?:(?!^###?\s).*\n)*)"
        )?;

        let pb = ProgressBar::new(task_regex.captures_iter(&content).count() as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
                .unwrap()
        );

        for cap in task_regex.captures_iter(&content) {
            self.stats.tasks_found += 1;

            let task_id = &cap[1];
            let title = &cap[2];
            let body = &cap[3];

            pb.set_message(format!("Processing {}", task_id));

            match self.parse_task(task_id, title, body) {
                Ok(task) => {
                    if !self.dry_run {
                        let service = crate::services::task_service::TaskService::new(self.db.clone());
                        if let Err(e) = service.create_task(task).await {
                            self.stats.errors.push(format!("Failed to create task {}: {}", task_id, e));
                        } else {
                            self.stats.tasks_migrated += 1;
                        }
                    } else {
                        println!("  Would create task: {} - {}", task_id, title);
                        self.stats.tasks_migrated += 1;
                    }
                }
                Err(e) => {
                    self.stats.errors.push(format!("Failed to parse task {}: {}", task_id, e));
                }
            }

            pb.inc(1);
        }

        pb.finish_with_message("Tasks migration complete");
        Ok(())
    }

    fn parse_task(&self, id: &str, title: &str, body: &str) -> Result<Task> {
        let mut task = Task::new(
            id.to_string(),
            title.trim().to_string(),
            TaskType::Task, // Default type
        );

        // Parse status
        if let Some(cap) = Regex::new(r"\*\*Status:\*\*\s*([^\n]+)")?.captures(body) {
            task.status = self.parse_status(&cap[1])?;
        }

        // Parse type
        if let Some(cap) = Regex::new(r"\*\*Type:\*\*\s*([^\n]+)")?.captures(body) {
            task.task_type = self.parse_task_type(&cap[1])?;
        }

        // Parse description
        if let Some(cap) = Regex::new(r"\*\*Description:\*\*\s*([^*]+)")?.captures(body) {
            task.description = Some(cap[1].trim().to_string());
        }

        // Parse assignee
        if let Some(cap) = Regex::new(r"\*\*Assignee:\*\*\s*([^\n]+)")?.captures(body) {
            task.assignee = Some(cap[1].trim().to_string());
        }

        // Parse sprint
        if let Some(cap) = Regex::new(r"\*\*Sprint:\*\*\s*(sprint-\d+)")?.captures(body) {
            task.sprint_id = Some(cap[1].to_string());
        }

        // Parse story points
        if let Some(cap) = Regex::new(r"\*\*Story Points:\*\*\s*(\d+)")?.captures(body) {
            task.story_points = cap[1].parse().ok();
        }

        // Parse dependencies
        if let Some(cap) = Regex::new(r"\*\*Dependencies:\*\*\s*([^\n]+)")?.captures(body) {
            task.dependencies = cap[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // Parse labels
        if let Some(cap) = Regex::new(r"\*\*Labels:\*\*\s*([^\n]+)")?.captures(body) {
            task.labels = cap[1]
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        Ok(task)
    }
}