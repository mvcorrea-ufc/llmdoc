// llmdoc/src/services/task_service.rs

use crate::core::errors::{Error, Result};
use async_trait::async_trait;
use std::sync::Arc;
use crate::core::database::DbConnection;
use crate::core::models::task::Task;

#[async_trait]
pub trait TaskServiceTrait: Send + Sync {
    async fn create_task(&self, task: Task) -> Result<Task>;
    async fn get_task_by_id(&self, id: String) -> Result<Option<Task>>;
    async fn update_task(&self, task: Task) -> Result<Task>;
    async fn delete_task(&self, id: String) -> Result<()>;
    async fn list_tasks(&self) -> Result<Vec<Task>>;
}

#[derive(Clone)]
pub struct TaskService {
    db_connection: Arc<DbConnection>,
}

impl TaskService {
    pub fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }
}

#[async_trait]
impl TaskServiceTrait for TaskService {
    async fn create_task(&self, task: Task) -> Result<Task> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let task_json = serde_json::to_string(&task)
            .map_err(|e| Error::JsonError(e))?;
        
        conn.interact(move |conn| {
            conn.execute(
                "INSERT INTO tasks (task_id, data) VALUES (?1, ?2)",
                rusqlite::params![&task.id, &task_json],
            )?;
            Ok(task)
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn get_task_by_id(&self, _id: String) -> Result<Option<Task>> {
        // Implementation placeholder
        Err(Error::OperationFailed("Not implemented".to_string()))
    }

    async fn update_task(&self, _task: Task) -> Result<Task> {
        // Implementation placeholder
        Err(Error::OperationFailed("Not implemented".to_string()))
    }

    async fn delete_task(&self, _id: String) -> Result<()> {
        // Implementation placeholder
        Err(Error::OperationFailed("Not implemented".to_string()))
    }

    async fn list_tasks(&self) -> Result<Vec<Task>> {
        // Implementation placeholder
        Err(Error::OperationFailed("Not implemented".to_string()))
    }
}