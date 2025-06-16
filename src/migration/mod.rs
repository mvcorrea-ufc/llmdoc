// llmdoc/src/migration/mod.rs

pub mod tasks;
pub mod sprints;
pub mod components;
pub mod adrs;
pub mod user_stories;

use crate::{
    core::{
        database::DbConnection,
        models::{TaskType, TaskStatus},
        errors::Result,
    },
};
use regex::Regex;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::info;

pub struct MarkdownMigrator {
    db: Arc<DbConnection>,
    docs_dir: PathBuf,
    dry_run: bool,
    stats: MigrationStats,
}

#[derive(Default, Debug)]
pub struct MigrationStats {
    pub tasks_found: usize,
    pub tasks_migrated: usize,
    pub sprints_found: usize,
    pub sprints_migrated: usize,
    pub components_found: usize,
    pub components_migrated: usize,
    pub adrs_found: usize,
    pub adrs_migrated: usize,
    pub stories_found: usize,
    pub stories_migrated: usize,
    pub errors: Vec<String>,
}

impl MarkdownMigrator {
    pub fn new(db: Arc<DbConnection>, docs_dir: PathBuf, dry_run: bool) -> Self {
        Self {
            db,
            docs_dir,
            dry_run,
            stats: MigrationStats::default(),
        }
    }

    pub async fn migrate(&mut self) -> Result<()> {
        info!("Starting migration from {:?}", self.docs_dir);

        if self.dry_run {
            println!("🔍 Running in DRY RUN mode - no changes will be made");
        }

        // Migrate different document types
        self.migrate_tasks().await?;
        self.migrate_sprints().await?;
        self.migrate_user_stories().await?;
        self.migrate_components().await?;
        self.migrate_adrs().await?;

        // Print report
        self.print_report();

        Ok(())
    }

    fn extract_section(&self, content: &str, section_name: &str) -> Option<String> {
        let pattern = format!(r"(?s)##\s*{}\s*\n(.*?)(?=\n##|\z)", regex::escape(section_name));
        Regex::new(&pattern).ok()
            .and_then(|re| re.captures(content))
            .map(|cap| cap[1].trim().to_string())
    }

    fn parse_status(&self, status_str: &str) -> Result<TaskStatus> {
        match status_str.trim().to_lowercase().as_str() {
            "todo" | "to do" | "to-do" => Ok(TaskStatus::Todo),
            "in progress" | "in_progress" | "in-progress" | "doing" => Ok(TaskStatus::InProgress),
            "done" | "completed" | "complete" => Ok(TaskStatus::Done),
            "blocked" => Ok(TaskStatus::Blocked),
            "cancelled" => Ok(TaskStatus::Cancelled),
            _ => Ok(TaskStatus::Todo), // Default
        }
    }

    fn parse_task_type(&self, type_str: &str) -> Result<TaskType> {
        match type_str.trim().to_lowercase().as_str() {
            "feature" => Ok(TaskType::Feature),
            "bug" | "bugfix" | "fix" => Ok(TaskType::Bug),
            "task" => Ok(TaskType::Task),
            "epic" => Ok(TaskType::Epic),
            "story" | "user story" | "userstory" => Ok(TaskType::Story),
            "spike" | "research" => Ok(TaskType::Spike),
            _ => Ok(TaskType::Task), // Default
        }
    }

    fn print_report(&self) {
        println!("\n📊 Migration Report");
        println!("{}", "=".repeat(50));

        println!("Tasks:       {} found, {} migrated", self.stats.tasks_found, self.stats.tasks_migrated);
        println!("Sprints:     {} found, {} migrated", self.stats.sprints_found, self.stats.sprints_migrated);
        println!("Stories:     {} found, {} migrated", self.stats.stories_found, self.stats.stories_migrated);
        println!("Components:  {} found, {} migrated", self.stats.components_found, self.stats.components_migrated);
        println!("ADRs:        {} found, {} migrated", self.stats.adrs_found, self.stats.adrs_migrated);

        if !self.stats.errors.is_empty() {
            println!("\n⚠️  Errors encountered:");
            for error in &self.stats.errors {
                println!("  - {}", error);
            }
        }

        let total_found = self.stats.tasks_found + self.stats.sprints_found +
                         self.stats.stories_found + self.stats.components_found +
                         self.stats.adrs_found;
        let total_migrated = self.stats.tasks_migrated + self.stats.sprints_migrated +
                            self.stats.stories_migrated + self.stats.components_migrated +
                            self.stats.adrs_migrated;

        println!("\n✅ Total: {} documents found, {} migrated successfully", total_found, total_migrated);

        if self.dry_run {
            println!("\n🔍 This was a DRY RUN - no changes were made");
            println!("   Run without --dry-run to perform the actual migration");
        }
    }
}

// CLI command implementation
pub async fn migrate_command(
    db: Arc<DbConnection>,
    docs_dir: PathBuf,
    dry_run: bool
) -> Result<()> {
    let mut migrator = MarkdownMigrator::new(db, docs_dir, dry_run);
    migrator.migrate().await
}