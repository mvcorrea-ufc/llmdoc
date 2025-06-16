use serde::Serialize;
// llmdoc/src/services/sprint_service.rs

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::core::database::DbConnection;
use crate::core::errors::{Error, Result};
use crate::core::models::sprint::{Sprint, SprintStatus, Retrospective};
use tracing::{debug, instrument};
use std::str::FromStr;

#[async_trait]
pub trait SprintServiceTrait: Send + Sync {
    async fn create_sprint(&self, sprint: Sprint) -> Result<Sprint>;
    async fn get_sprint_by_id(&self, id: &str) -> Result<Option<Sprint>>;
    async fn update_sprint(&self, sprint: Sprint) -> Result<Sprint>;
    async fn delete_sprint(&self, id: &str) -> Result<()>;
    async fn list_sprints(&self, status: Option<SprintStatus>) -> Result<Vec<Sprint>>;
    async fn get_current_sprint(&self) -> Result<Option<Sprint>>;
    async fn start_sprint(&self, sprint_id: &str) -> Result<Sprint>;
    async fn complete_sprint(&self, sprint_id: &str, retrospective: Option<Retrospective>) -> Result<Sprint>;
    async fn get_sprint_report(&self, sprint_id: &str) -> Result<SprintReport>;
}

#[derive(Clone)]
pub struct SprintService {
    db_connection: Arc<DbConnection>,
}

impl SprintService {
    pub fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl SprintServiceTrait for SprintService {

    #[instrument(skip(self, sprint), err)]
    async fn create_sprint(&self, sprint: Sprint) -> Result<Sprint> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool: {}", e)))?;
        let sprint_clone = sprint.clone();
        conn.interact(move |conn| {
            let mut stmt = conn.prepare(
                "INSERT INTO sprints (id, name, description, start_date, end_date, goals, tasks, status, velocity, capacity, created_at, updated_at, retrospective) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )?;
            
            // Convert values to appropriate types for rusqlite
            let name = sprint_clone.name.clone();
            let description = sprint_clone.description.clone(); // Clone to avoid partial move
            let start_date = sprint_clone.start_date.to_rfc3339();
            let end_date = sprint_clone.end_date.to_rfc3339();
            let goals = serde_json::to_string(&sprint_clone.goals)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let tasks = serde_json::to_string(&sprint_clone.tasks)
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
            let status = sprint_clone.status.to_string();
            let velocity = sprint_clone.velocity.map(|v| v.to_string());
            let capacity = sprint_clone.capacity.map(|c| c.to_string());
            let created_at = sprint_clone.created_at.to_rfc3339();
            let updated_at = sprint_clone.updated_at.to_rfc3339();
            let retrospective = sprint_clone.retrospective
                .as_ref()
                .and_then(|r| serde_json::to_string(r).ok());

            // Convert all fields to owned values for SQLite
            let id_val: String = sprint_clone.id.clone();
            let name_val: String = name.clone();
            let description_val = description.as_deref().unwrap_or(""); // Use as_deref on Option<String>
            let start_date_val: String = start_date.clone();
            let end_date_val: String = end_date.clone();
            let goals_val: String = goals.clone();
            let tasks_val: String = tasks.clone();
            let status_val: String = status.clone();
            let velocity_val: String = velocity.as_ref().map(|s| s.to_string()).unwrap_or_default();
            let capacity_val: String = capacity.as_ref().map(|s| s.to_string()).unwrap_or_default();
            let created_at_val: String = created_at.clone();
            let updated_at_val: String = updated_at.clone();
            let retrospective_val: String = retrospective.as_ref().map(|s| s.to_string()).unwrap_or_default();

            stmt.execute(rusqlite::params![
                &id_val,
                &name_val,
                &description_val,
                &start_date_val,
                &end_date_val,
                &goals_val,
                &tasks_val,
                &status_val,
                &velocity_val,
                &capacity_val,
                &created_at_val,
                &updated_at_val,
                &retrospective_val,
            ])?;
            debug!("Created sprint: {}", sprint_clone.id);
            Ok(sprint_clone)
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Deadpool interact error: {:?}", e)))?
        .map_err(|e: rusqlite::Error| Error::DatabaseOperation(format!("Failed to create sprint: {}", e)))
    }

    #[instrument(skip(self), err)]
    async fn get_sprint_by_id(&self, id: &str) -> Result<Option<Sprint>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool: {}", e)))?;
        let id_clone = id.to_string();
        conn.interact(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, description, start_date, end_date, goals, tasks, status, velocity, capacity, created_at, updated_at, retrospective FROM sprints WHERE id = ?"
            )?;
            let sprint_row = stmt.query_row([id_clone], |row| {
                let description: Option<String> = row.get(2)?;
                let start_date_str: String = row.get(3)?;
                let end_date_str: String = row.get(4)?;
                let goals_str: String = row.get(5)?;
                let tasks_str: String = row.get(6)?;
                let status_str: String = row.get(7)?;
                let velocity: Option<f32> = row.get(8)?;
                let capacity: Option<f32> = row.get(9)?;
                let created_at_str: String = row.get(10)?;
                let updated_at_str: String = row.get(11)?;
                let retrospective_str: Option<String> = row.get(12)?;

                Ok(Sprint {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description,
                    start_date: DateTime::parse_from_rfc3339(&start_date_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    end_date: DateTime::parse_from_rfc3339(&end_date_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    goals: serde_json::from_str(&goals_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?, // Corrected error mapping
                    tasks: serde_json::from_str(&tasks_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?, // Corrected error mapping
                    status: SprintStatus::from_str(&status_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::InvalidInput(e.to_string()))))?, // Corrected error mapping
                    velocity,
                    capacity,
                    created_at: DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    retrospective: retrospective_str
                        .map(|s| serde_json::from_str(&s))
                        .transpose()
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?, // Corrected error mapping
                })
            });
            match sprint_row {
                Ok(s) => Ok(Some(s)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e),
            }
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Deadpool interact error: {:?}", e)))?
        .map_err(|e| Error::DatabaseOperation(format!("Failed to get sprint by ID: {}", e)))
    }

    #[instrument(skip(self, sprint), err)]
    async fn update_sprint(&self, sprint: Sprint) -> Result<Sprint> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool: {}", e)))?;
        let sprint_clone = sprint.clone();
        conn.interact(move |conn| {
            let mut stmt = conn.prepare(
                "UPDATE sprints SET name = ?, description = ?, start_date = ?, end_date = ?, goals = ?, tasks = ?, status = ?, velocity = ?, capacity = ?, updated_at = ?, retrospective = ? WHERE id = ?"
            )?;
            // Convert numeric options to strings with explicit types
            let velocity_value: String = sprint_clone.velocity.map(|v| v.to_string()).unwrap_or_default();
            let capacity_value: String = sprint_clone.capacity.map(|c| c.to_string()).unwrap_or_default();
            let retrospective_value: String = sprint_clone.retrospective
                .as_ref()
                .map(|r| serde_json::to_string(r))
                .transpose()
                .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))? // Corrected error mapping
                .unwrap_or_default();

            stmt.execute(rusqlite::params![
                &sprint_clone.name,
                sprint_clone.description.as_deref().unwrap_or(""),
                &sprint_clone.start_date.to_rfc3339(),
                &sprint_clone.end_date.to_rfc3339(),
                &serde_json::to_string(&sprint_clone.goals)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?, // Corrected error mapping
                &serde_json::to_string(&sprint_clone.tasks)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?, // Corrected error mapping
                &sprint_clone.status.to_string(),
                &velocity_value,
                &capacity_value,
                &sprint_clone.updated_at.to_rfc3339(),
                &retrospective_value,
                &sprint_clone.id,
            ])?;
            debug!("Updated sprint: {}", sprint_clone.id);
            Ok(sprint_clone)
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Deadpool interact error: {:?}", e)))?
        .map_err(|e: rusqlite::Error| Error::DatabaseOperation(format!("Failed to update sprint: {}", e)))
    }

    #[instrument(skip(self), err)]
    async fn delete_sprint(&self, id: &str) -> Result<()> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool: {}", e)))?;
        let id_owned = id.to_string(); // Clone id to an owned String
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("DELETE FROM sprints WHERE id = ?")?;
            let id_str = id_owned.clone();
            stmt.execute([&id_str])?;
            debug!("Deleted sprint: {}", id_str);
            Ok::<(), rusqlite::Error>(())
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Deadpool interact error: {:?}", e)))?
        .map_err(|e| Error::DatabaseOperation(format!("Failed to delete sprint: {}", e)))
    }

    #[instrument(skip(self), err)]
    async fn get_current_sprint(&self) -> Result<Option<Sprint>> {
        // Placeholder: In a real app, this would involve querying for an active sprint
        // based on dates or a dedicated 'current' flag.
        // For now, we'll just return the most recently created sprint as a mock "current".
        let conn = self.db_connection.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool: {}", e)))?;
        conn.interact(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT id, name, description, start_date, end_date, goals, tasks, status, velocity, capacity, created_at, updated_at, retrospective FROM sprints ORDER BY created_at DESC LIMIT 1"
            )?;
            let sprint_row = stmt.query_row([], |row| {
                let description: Option<String> = row.get(2)?;
                let start_date_str: String = row.get(3)?;
                let end_date_str: String = row.get(4)?;
                let goals_str: String = row.get(5)?;
                let tasks_str: String = row.get(6)?;
                let status_str: String = row.get(7)?;
                let velocity: Option<f32> = row.get(8)?;
                let capacity: Option<f32> = row.get(9)?;
                let created_at_str: String = row.get(10)?;
                let updated_at_str: String = row.get(11)?;
                let retrospective_str: Option<String> = row.get(12)?;

                Ok(Sprint {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description,
                    start_date: DateTime::parse_from_rfc3339(&start_date_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))?
                        .with_timezone(&Utc),
                    end_date: DateTime::parse_from_rfc3339(&end_date_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
                        .with_timezone(&Utc),
                    goals: serde_json::from_str(&goals_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?,
                    tasks: serde_json::from_str(&tasks_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?,
                    status: SprintStatus::from_str(&status_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::InvalidInput(e.to_string()))))?,
                    velocity,
                    capacity,
                    created_at: DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(e)))?
                        .with_timezone(&Utc),
                    retrospective: retrospective_str
                        .map(|s| serde_json::from_str(&s))
                        .transpose()
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?,
                })
            });
            match sprint_row {
                Ok(s) => Ok(Some(s)),
                Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
                Err(e) => Err(e),
            }
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Deadpool interact error: {:?}", e)))?
        .map_err(|e| Error::DatabaseOperation(format!("Failed to get current sprint: {}", e)))
    }

    #[instrument(skip(self), err)]
    async fn list_sprints(&self, status: Option<SprintStatus>) -> Result<Vec<Sprint>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection from pool: {}", e)))?;
        conn.interact(move |conn| {
            let mut query = "SELECT id, name, description, start_date, end_date, goals, tasks, status, velocity, capacity, created_at, updated_at, retrospective FROM sprints".to_string();
            let mut params: Vec<&dyn rusqlite::ToSql> = Vec::new();

            let status_string;
            if let Some(sprint_status) = status {
                query.push_str(" WHERE status = ?");
                status_string = sprint_status.to_string();
                params.push(&status_string);
            }

            let mut stmt = conn.prepare(&query)?;
            let sprints_iter = stmt.query_map(&*params, |row| {
                let description: Option<String> = row.get(2)?;
                let start_date_str: String = row.get(3)?;
                let end_date_str: String = row.get(4)?;
                let goals_str: String = row.get(5)?;
                let tasks_str: String = row.get(6)?;
                let status_str: String = row.get(7)?;
                let velocity: Option<f32> = row.get(8)?;
                let capacity: Option<f32> = row.get(9)?;
                let created_at_str: String = row.get(10)?;
                let updated_at_str: String = row.get(11)?;
                let retrospective_str: Option<String> = row.get(12)?;

                Ok(Sprint {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    description,
                    start_date: DateTime::parse_from_rfc3339(&start_date_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    end_date: DateTime::parse_from_rfc3339(&end_date_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    goals: serde_json::from_str(&goals_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?, // Corrected error mapping
                    tasks: serde_json::from_str(&tasks_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?, // Corrected error mapping
                    status: SprintStatus::from_str(&status_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::InvalidInput(e.to_string()))))?, // Corrected error mapping
                    velocity,
                    capacity,
                    created_at: DateTime::parse_from_rfc3339(&created_at_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(&updated_at_str)
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::Parse(e))))? // Corrected error mapping
                        .with_timezone(&Utc),
                    retrospective: retrospective_str
                        .map(|s| serde_json::from_str(&s))
                        .transpose()
                        .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(Error::JsonError(e))))?, // Corrected error mapping
                })
            })?;

            let mut sprints = Vec::new();
            for sprint in sprints_iter {
                sprints.push(sprint?);
            }
            Ok(sprints)
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Deadpool interact error: {:?}", e)))?
        .map_err(|e: rusqlite::Error| Error::DatabaseOperation(format!("Failed to list sprints: {}", e)))
    }

    #[instrument(skip(self), err)]
    async fn start_sprint(&self, sprint_id: &str) -> Result<Sprint> {
        let mut sprint = self.get_sprint_by_id(sprint_id).await?
            .ok_or_else(|| Error::NotFound(format!("Sprint with ID {} not found", sprint_id)))?;
        sprint.status = SprintStatus::Active;
        sprint.updated_at = Utc::now();
        self.update_sprint(sprint).await
    }

    #[instrument(skip(self, retrospective), err)]
    async fn complete_sprint(&self, sprint_id: &str, retrospective: Option<Retrospective>) -> Result<Sprint> {
        let mut sprint = self.get_sprint_by_id(sprint_id).await?
            .ok_or_else(|| Error::NotFound(format!("Sprint with ID {} not found", sprint_id)))?;
        sprint.status = SprintStatus::Completed;
        sprint.retrospective = retrospective;
        sprint.updated_at = Utc::now();
        self.update_sprint(sprint).await
    }

    #[instrument(skip(self), err)]
    async fn get_sprint_report(&self, sprint_id: &str) -> Result<SprintReport> {
        let sprint = self.get_sprint_by_id(sprint_id).await?.ok_or_else(|| Error::NotFound(format!("Sprint with ID {} not found", sprint_id)))?;
        
        // TODO: Implement actual report generation logic
        // Placeholder implementation - needs real data
        Ok(SprintReport {
            sprint_id: sprint.id,
            sprint_name: sprint.name,
            total_tasks: sprint.tasks.len() as u32,
            completed_tasks: sprint.tasks.iter().filter(|t| t.completed_at.is_some()).count() as u32,
            remaining_tasks: sprint.tasks.iter().filter(|t| t.completed_at.is_none()).count() as u32,
            total_story_points: sprint.tasks.iter().map(|t| t.story_points.unwrap_or(0) as u32).sum(),
            completed_story_points: sprint.tasks.iter().filter(|t| t.completed_at.is_some()).map(|t| t.story_points.unwrap_or(0) as u32).sum(),
            remaining_story_points: sprint.tasks.iter().filter(|t| t.completed_at.is_none()).map(|t| t.story_points.unwrap_or(0) as u32).sum(),
            retrospective: sprint.retrospective.clone(),
        })
    }

}

// Placeholder for SprintReport struct
#[derive(Debug, Serialize)]
pub struct SprintReport {
    pub sprint_id: String,
    pub sprint_name: String,
    pub total_tasks: u32,
    pub completed_tasks: u32,
    pub remaining_tasks: u32,
    pub total_story_points: u32,
    pub completed_story_points: u32,
    pub remaining_story_points: u32,
    pub retrospective: Option<Retrospective>,
}