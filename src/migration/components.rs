// llmdoc/src/migration/components.rs

use crate::{
    core::{
        models::{Component, ComponentType},
        errors::Result,
    },
};
use regex::Regex;
use std::collections::HashMap;
use tracing::{warn};
use serde_json;

use super::MarkdownMigrator; // Import from the parent module

impl MarkdownMigrator {
    pub(super) async fn migrate_components(&mut self) -> Result<()> {
        let components_dir = self.docs_dir.join("components");
        if !components_dir.exists() {
            warn!("components directory not found");
            return Ok(());
        }

        println!("🔧 Migrating components...");

        for entry in std::fs::read_dir(&components_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md") {
                self.stats.components_found += 1;

                let content = std::fs::read_to_string(&path)?;
                let comp_id = format!(
                    "comp-{}",
                    path.file_stem()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .replace('_', "-")
                );

                let mut component = Component {
                    id: comp_id.clone(),
                    name: comp_id.clone(),
                    component_type: ComponentType::Module,
                    description: String::new(),
                    dependencies: vec![],
                    interfaces: vec![],
                    tech_stack: vec![],
                    owner: None,
                    documentation_url: None,
                    repository_url: None,
                    created_at: chrono::Utc::now(), // Use chrono::Utc
                    updated_at: chrono::Utc::now(), // Use chrono::Utc
                    metadata: HashMap::new(),
                };

                // Parse component name
                if let Some(cap) = Regex::new(r"^#\s+(.+)")?.captures(&content) {
                    component.name = cap[1].trim().to_string();
                }

                // Parse overview/description
                if let Some(overview) = self.extract_section(&content, "Overview") {
                    component.description = overview;
                } else if let Some(purpose) = self.extract_section(&content, "Purpose") {
                    component.description = purpose;
                }

                // Parse dependencies
                if let Some(deps_section) = self.extract_section(&content, "Dependencies") {
                    component.dependencies = deps_section
                        .lines()
                        .filter_map(|line| {
                            let line = line.trim();
                            if line.starts_with('-') || line.starts_with('*') {
                                Some(line[1..].trim().to_string())
                            } else {
                                None
                            }
                        })
                        .collect();
                }

                if !self.dry_run {
                    let component_clone = component.clone(); // Clone the component for the async block
                     let conn = self.db.pool.get().await.map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection from pool for component transaction: {}", e)))?;
                     conn.interact(move |conn| {
                         let json = serde_json::to_string(&component_clone).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?; // Map serde_json::Error to rusqlite::Error
                          conn.execute(
                             "INSERT INTO components (component_id, data) VALUES (?1, ?2)",
                             rusqlite::params![&component_clone.id, &json],
                          )?;
                          Ok::<(), rusqlite::Error>(()) // Explicit type annotation
                     }).await.map_err(|e| crate::core::errors::Error::DatabaseOperation(e.to_string()))??; // Added extra ? for conversion
                     self.stats.components_migrated += 1;
                 } else {
                     println!("  Would create component: {}", comp_id);
                    self.stats.components_migrated += 1;
                }
            }
        }

        Ok(())
    }
}