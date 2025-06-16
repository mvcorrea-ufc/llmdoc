// llmdoc/src/migration/sprints.rs

use crate::{
    core::{
        models::{Sprint, SprintStatus},
        errors::{Result, Error},
    },
    services::{
        sprint_service::{SprintService, SprintServiceTrait},
    },
    migration::MarkdownMigrator,
};
use chrono::{DateTime, NaiveDate, Utc};
use regex::Regex;
use std::sync::Arc;
use std::path::Path;

use tracing::warn;

impl MarkdownMigrator {
    pub(super) async fn migrate_sprints(&mut self) -> Result<()> {
        let sprints_dir = self.docs_dir.join("agile/sprints");
        if !sprints_dir.exists() {
            warn!("sprints directory not found");
            return Ok(());
        }

        println!("🏃 Migrating sprints...");

        // Current sprint
        let current_sprint_file = sprints_dir.join("sprint-current-plan.md");
        if current_sprint_file.exists() {
            self.migrate_sprint_file(&current_sprint_file, SprintStatus::Active).await?;
        }

        // Archived sprints
        let archive_dir = sprints_dir.join("archive");
        if archive_dir.exists() {
            for entry in std::fs::read_dir(&archive_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    self.migrate_sprint_file(&path, SprintStatus::Completed).await?;
                }
            }
        }

        Ok(())
    }

    async fn migrate_sprint_file(&mut self, file_path: &Path, status: SprintStatus) -> Result<()> {
        self.stats.sprints_found += 1;

        let content = std::fs::read_to_string(file_path)?;

        // Extract sprint ID from filename
        let sprint_id = if let Some(cap) = Regex::new(r"sprint-(\d+)")?.captures(
            file_path.file_name().unwrap().to_str().unwrap()
        ) {
            format!("sprint-{}", &cap[1])
        } else {
            return Err(Error::InvalidInput("Could not extract sprint ID from filename".into()));
        };

        // Parse sprint details
        let mut sprint = Sprint {
            id: sprint_id.clone(),
            name: format!("Sprint {}", sprint_id.split('-').last().unwrap()),
            description: None,
            start_date: Utc::now(),
            end_date: Utc::now(),
            goals: vec![],
            tasks: vec![],
            status,
            velocity: None,
            capacity: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            retrospective: None,
        };

        // Parse sprint name
        if let Some(cap) = Regex::new(r"#\s*(.+)")?.captures(&content) {
            sprint.name = cap[1].trim().to_string();
        }

        // Parse dates
        if let Some(cap) = Regex::new(r"(\d{4}-\d{2}-\d{2})\s*-\s*(\d{4}-\d{2}-\d{2})")?.captures(&content) {
            sprint.start_date = DateTime::from_naive_utc_and_offset(
                NaiveDate::parse_from_str(&cap[1], "%Y-%m-%d")
                    .map_err(|e| Error::Parse(e))? // Corrected error mapping
                    .and_hms_opt(0, 0, 0)
                    .ok_or_else(|| Error::InvalidInput("Invalid time components for start date".to_string()))?, // Corrected error mapping
                Utc,
            );
            sprint.end_date = DateTime::from_naive_utc_and_offset(
                NaiveDate::parse_from_str(&cap[2], "%Y-%m-%d")
                    .map_err(|e| Error::Parse(e))? // Corrected error mapping
                    .and_hms_opt(23, 59, 59)
                    .ok_or_else(|| Error::InvalidInput("Invalid time components for end date".to_string()))?, // Corrected error mapping
                Utc,
            );
        }

        // Parse goals
        if let Some(goals_section) = Regex::new(r"(?s)##\s*Goals?\s*\n(.*?)(?=##|\z)")?.captures(&content) {
            sprint.goals = goals_section[1]
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.starts_with('-') || line.starts_with('*') {
                        Some(line[1..].trim().to_string())
                    } else {
                        None
                    }
                })
                .filter(|s| !s.is_empty())
                .collect();
        }

        // Extract task IDs
        let task_regex = Regex::new(r"\b([A-Z]+-\d+)\b")?;
        let mut task_ids: Vec<String> = task_regex
            .captures_iter(&content)
            .map(|cap| cap[1].to_string())
            .collect();
        task_ids.sort();
        task_ids.dedup();
        // Temporary fix for type mismatch - will need proper task resolution later
        // sprint.tasks = task_ids;
        warn!("Temporarily skipping task assignments for sprint {}", sprint_id);

        if !self.dry_run {
            let service = Arc::new(SprintService::new(self.db.clone()));
            if let Err(e) = service.create_sprint(sprint).await {
                self.stats.errors.push(format!("Failed to create sprint {}: {}", sprint_id, e));
            } else {
                self.stats.sprints_migrated += 1;
            }
        } else {
            println!("  Would create sprint: {}", sprint_id);
            self.stats.sprints_migrated += 1;
        }

        Ok(())
    }
}