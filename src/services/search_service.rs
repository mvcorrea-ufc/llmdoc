// llmdoc/src/services/search_service.rs

use crate::core::errors::Result;
use std::sync::Arc;
use async_trait::async_trait;
use crate::core::database::DbConnection;
use crate::core::models::task::Task; // Example model to search

#[async_trait]
pub trait SearchServiceTrait: Send + Sync {
    fn new(db_connection: Arc<DbConnection>) -> Self;
    async fn search_tasks(&self, query: &str) -> Result<Vec<Task>>;
    // Add other search methods as needed
}

#[derive(Clone)]
pub struct SearchService {
    db_connection: Arc<DbConnection>,
}

#[async_trait]
impl SearchServiceTrait for SearchService {
    fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }

    async fn search_tasks(&self, query: &str) -> Result<Vec<Task>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let search_query = format!("%{}%", query);
        
        conn.interact(move |conn| {
            let mut stmt = conn.prepare(
                "SELECT data FROM tasks WHERE data LIKE ?1"
            )?;
            let task_iter = stmt.query_map([&search_query], |row| {
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
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }
}