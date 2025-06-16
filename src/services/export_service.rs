// llmdoc/src/services/export_service.rs

use crate::core::errors::Result;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::database::DbConnection;
use crate::core::models::task::Task; // Example model to export
use std::fs;
use std::path::Path;

#[async_trait]
pub trait ExportServiceTrait: Send + Sync {
    fn new(db_connection: Arc<DbConnection>) -> Self;
    async fn export_tasks_to_markdown(&self, tasks: Vec<Task>, path: &str) -> Result<()>;
    async fn export_all_tasks_to_markdown(&self, path: &str) -> Result<()>;
    // Add other export methods as needed
}

#[derive(Clone)]
pub struct ExportService {
    db_connection: Arc<DbConnection>,
}

#[async_trait]
impl ExportServiceTrait for ExportService {
    fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }

    async fn export_tasks_to_markdown(&self, tasks: Vec<Task>, path: &str) -> Result<()> {
        let mut markdown_content = String::new();
        markdown_content.push_str("# Tasks Export\n\n");
        
        for task in tasks {
            markdown_content.push_str(&format!("## {}\n\n", task.title));
            markdown_content.push_str(&format!("**ID:** {}\n\n", task.id));
            markdown_content.push_str(&format!("**Status:** {:?}\n\n", task.status));
            markdown_content.push_str(&format!("**Type:** {:?}\n\n", task.task_type));
            markdown_content.push_str(&format!("**Priority:** {:?}\n\n", task.priority));
            
            if let Some(description) = &task.description {
                markdown_content.push_str(&format!("**Description:** {}\n\n", description));
            }
            
            if let Some(assignee) = &task.assignee {
                markdown_content.push_str(&format!("**Assignee:** {}\n\n", assignee));
            }
            
            if let Some(sprint_id) = &task.sprint_id {
                markdown_content.push_str(&format!("**Sprint:** {}\n\n", sprint_id));
            }
            
            if let Some(points) = task.story_points {
                markdown_content.push_str(&format!("**Story Points:** {}\n\n", points));
            }
            
            if !task.labels.is_empty() {
                markdown_content.push_str(&format!("**Labels:** {}\n\n", task.labels.join(", ")));
            }
            
            markdown_content.push_str(&format!("**Created:** {}\n\n", task.created_at.format("%Y-%m-%d %H:%M:%S")));
            markdown_content.push_str(&format!("**Updated:** {}\n\n", task.updated_at.format("%Y-%m-%d %H:%M:%S")));
            
            markdown_content.push_str("---\n\n");
        }
        
        // Ensure the directory exists
        if let Some(parent) = Path::new(path).parent() {
            fs::create_dir_all(parent)
                .map_err(|e| crate::core::errors::Error::IoError(e))?;
        }
        
        fs::write(path, markdown_content)
            .map_err(|e| crate::core::errors::Error::IoError(e))?;
            
        Ok(())
    }
    
    async fn export_all_tasks_to_markdown(&self, path: &str) -> Result<()> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let tasks = conn.interact(move |conn| {
            let mut stmt = conn.prepare("SELECT data FROM tasks")?;
            let task_iter = stmt.query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;
            
            let mut tasks = Vec::new();
            for task_result in task_iter {
                let data = task_result?;
                let task: Task = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                tasks.push(task);
            }
            
            Ok(tasks)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))?;
        
        self.export_tasks_to_markdown(tasks, path).await
    }
}