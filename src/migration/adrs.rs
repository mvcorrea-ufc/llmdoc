// llmdoc/src/migration/adrs.rs

use crate::{
    core::{
        models::{Adr, AdrStatus},
        errors::{Result, Error},
    },
};
use regex::Regex;
use tracing::warn;
use serde_json;

use super::MarkdownMigrator; // Import from the parent module

impl MarkdownMigrator {
    pub(super) async fn migrate_adrs(&mut self) -> Result<()> {
        let adr_dir = self.docs_dir.join("architecture");
        if !adr_dir.exists() {
            warn!("architecture directory not found");
            return Ok(());
        }

        println!("📐 Migrating ADRs...");

        for entry in std::fs::read_dir(&adr_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("md")
                && path.file_name().unwrap().to_str().unwrap().starts_with("ADR")
            {
                self.stats.adrs_found += 1;

                let content = std::fs::read_to_string(&path)?;

                // Extract ADR number
                let adr_id = if let Some(cap) = Regex::new(r"ADR(\d+)")?.captures(
                    path.file_name().unwrap().to_str().unwrap()
                ) {
                    format!("ADR{:03}", cap[1].parse::<u32>()
                        .map_err(|e| Error::ParseIntError(e))?) // Corrected error mapping
                } else {
                    continue;
                };

                let mut adr = Adr {
                    id: adr_id.clone(),
                    title: String::new(),
                    status: AdrStatus::Accepted,
                    context: String::new(),
                    decision: String::new(),
                    consequences: String::new(),
                    alternatives: vec![],
                    related_adrs: vec![],
                    created_at: chrono::Utc::now(), // Use chrono::Utc
                    updated_at: chrono::Utc::now(), // Use chrono::Utc
                    created_by: None,
                    approved_by: None,
                    approved_at: None,
                };

                // Parse title
                if let Some(cap) = Regex::new(r"^#\s+(.+)")?.captures(&content) {
                    adr.title = cap[1].trim().to_string();
                }

                // Parse sections
                adr.context = self.extract_section(&content, "Context").unwrap_or_default();
                adr.decision = self.extract_section(&content, "Decision").unwrap_or_default();
                adr.consequences = self.extract_section(&content, "Consequences").unwrap_or_default();

                // Parse alternatives
                if let Some(alts) = self.extract_section(&content, "Alternatives") {
                    adr.alternatives = alts
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
                    let adr_clone = adr.clone();
                    let conn = self.db.pool.get().await.map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool for ADR transaction: {}", e)))?;
                    conn.interact(move |conn| {
                        let json = serde_json::to_string(&adr_clone).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?; // Map serde_json::Error to rusqlite::Error
                        conn.execute(
                            "INSERT INTO adrs (adr_id, data, status) VALUES (?1, ?2, ?3)",
                            rusqlite::params![&adr_clone.id, &json, "accepted"],
                        )?;
                        Ok::<(), rusqlite::Error>(()) // Explicit type annotation
                    }).await.map_err(|e| Error::DatabaseOperation(e.to_string()))??; // Added extra ? for conversion
                    self.stats.adrs_migrated += 1;
                } else {
                    println!("  Would create ADR: {} - {}", adr_id, adr.title);
                    self.stats.adrs_migrated += 1;
                }
            }
        }

        Ok(())
    }
}