// llmdoc/src/migration/user_stories.rs

use crate::{
    core::{
        models::{UserStory, Priority},
        errors::{Result, Error},
    },
};
use regex::Regex;
use tracing::warn;
use serde_json;

use super::MarkdownMigrator; // Import from the parent module

impl MarkdownMigrator {
    pub(super) async fn migrate_user_stories(&mut self) -> Result<()> {
        let stories_file = self.docs_dir.join("agile/user-stories.md");
        if !stories_file.exists() {
            warn!("user-stories.md not found");
            return Ok(());
        }

        println!("📖 Migrating user stories...");
        let content = std::fs::read_to_string(&stories_file)?;

        // Parse story sections
        let story_regex = Regex::new(r"(?m)^##\s+(.+?)$\n((?:(?!^##\s).*\n)*)")?;

        let mut story_counter = 1;
        for cap in story_regex.captures_iter(&content) {
            self.stats.stories_found += 1;

            let title = &cap[1];
            let body = &cap[2];

            let story_id = format!("US-{:03}", story_counter);
            story_counter += 1;

            // Parse user story pattern
            let story_pattern = Regex::new(
                r"(?i)as\s+a[n]?\s+(.+?),?\s+i\s+want\s+(.+?)\s+so\s+that\s+(.+?)(?:\.|$)"
            )?;

            let mut story = UserStory {
                id: story_id.clone(),
                title: title.trim().to_string(),
                persona: "user".to_string(),
                want: title.trim().to_string(),
                benefit: "value is delivered".to_string(),
                description: None, // Added missing field
                acceptance_criteria: vec![],
                story_points: None,
                priority: Priority::Medium,
                epic_id: None,
                tasks: vec![],
                created_at: chrono::Utc::now(), // Use chrono::Utc
                updated_at: chrono::Utc::now(), // Use chrono::Utc
            };

            if let Some(cap) = story_pattern.captures(body) {
                story.persona = cap[1].trim().to_string();
                story.want = cap[2].trim().to_string();
                story.benefit = cap[3].trim().to_string();
            }

            // Parse acceptance criteria
            if let Some(ac_section) = Regex::new(r"(?s)Acceptance Criteria:?\s*\n(.*?)(?=\n##|\z)")?.captures(body) {
                story.acceptance_criteria = ac_section[1]
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.starts_with('-') || line.starts_with('*') || line.starts_with("[ ]") {
                            Some(line.trim_start_matches('-')
                                .trim_start_matches('*')
                                .trim_start_matches("[ ]")
                                .trim()
                                .to_string())
                        } else {
                            None
                        }
                    })
                    .filter(|s| !s.is_empty())
                    .collect();
            }

            if !self.dry_run {
                // Store user story in database
                let story_clone = story.clone();
                let conn = self.db.pool.get().await.map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool for user story transaction: {}", e)))?;
                conn.interact(move |conn| {
                    let json = serde_json::to_string(&story_clone).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?; // Map serde_json::Error to rusqlite::Error
                    conn.execute(
                        "INSERT INTO user_stories (story_id, data) VALUES (?1, ?2)",
                        rusqlite::params![&story_clone.id, &json],
                    )?;
                    Ok::<(), rusqlite::Error>(()) // Explicit type annotation
                }).await.map_err(|e| Error::DatabaseOperation(e.to_string()))??; // Added extra ? for conversion
                self.stats.stories_migrated += 1;
            } else {
                println!("  Would create story: {} - {}", story_id, title);
                self.stats.stories_migrated += 1;
            }
        }

        Ok(())
    }
}