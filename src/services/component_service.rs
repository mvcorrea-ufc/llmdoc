// llmdoc/src/services/component_service.rs

use crate::core::errors::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::core::database::DbConnection;
use crate::core::models::component::Component;

#[async_trait]
pub trait ComponentServiceTrait: Send + Sync {
    fn new(db_connection: Arc<DbConnection>) -> Self;
    async fn create_component(&self, component: Component) -> Result<Component>;
    async fn get_component_by_id(&self, id: &str) -> Result<Option<Component>>;
    async fn update_component(&self, component: Component) -> Result<Component>;
    async fn delete_component(&self, id: &str) -> Result<()>;
    async fn list_components(&self) -> Result<Vec<Component>>;
}

#[derive(Clone)]
pub struct ComponentService {
    db_connection: Arc<DbConnection>,
}

#[async_trait]
impl ComponentServiceTrait for ComponentService {
    fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }

    async fn create_component(&self, component: Component) -> Result<Component> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let component_json = serde_json::to_string(&component)?;
        
        conn.interact(move |conn| {
            conn.execute(
                "INSERT INTO components (component_id, data) VALUES (?1, ?2)",
                rusqlite::params![&component.id, &component_json],
            )?;
            Ok(component)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn get_component_by_id(&self, id: &str) -> Result<Option<Component>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let component_id = id.to_string();
        
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("SELECT data FROM components WHERE component_id = ?1")?;
            let mut rows = stmt.query_map([&component_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;
            
            if let Some(row) = rows.next() {
                let data = row?;
                let component: Component = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                Ok(Some(component))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn update_component(&self, component: Component) -> Result<Component> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let component_json = serde_json::to_string(&component)?;
        let component_id = component.id.clone();
        
        conn.interact(move |conn| {
            conn.execute(
                "UPDATE components SET data = ?1 WHERE component_id = ?2",
                rusqlite::params![&component_json, &component_id],
            )?;
            Ok(component)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn delete_component(&self, id: &str) -> Result<()> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let component_id = id.to_string();
        
        conn.interact(move |conn| {
            conn.execute(
                "DELETE FROM components WHERE component_id = ?1",
                rusqlite::params![&component_id],
            )?;
            Ok(())
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn list_components(&self) -> Result<Vec<Component>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("SELECT data FROM components")?;
            let component_iter = stmt.query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;
            
            let mut components = Vec::new();
            for component_result in component_iter {
                let data = component_result?;
                let component: Component = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                components.push(component);
            }
            
            Ok(components)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }
}