// llmdoc/src/services/adr_service.rs

use crate::core::errors::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::core::database::DbConnection;
use crate::core::models::adr::Adr;

#[async_trait]
pub trait AdrServiceTrait: Send + Sync {
    fn new(db_connection: Arc<DbConnection>) -> Self;
    async fn create_adr(&self, adr: Adr) -> Result<Adr>;
    async fn get_adr_by_id(&self, id: &str) -> Result<Option<Adr>>;
    async fn update_adr(&self, adr: Adr) -> Result<Adr>;
    async fn delete_adr(&self, id: &str) -> Result<()>;
    async fn list_adrs(&self) -> Result<Vec<Adr>>;
}

#[derive(Clone)]
pub struct AdrService {
    db_connection: Arc<DbConnection>,
}

#[async_trait]
impl AdrServiceTrait for AdrService {
    fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }

    async fn create_adr(&self, adr: Adr) -> Result<Adr> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let adr_json = serde_json::to_string(&adr)?;
        
        conn.interact(move |conn| {
            conn.execute(
                "INSERT INTO adrs (adr_id, data, status) VALUES (?1, ?2, ?3)",
                rusqlite::params![&adr.id, &adr_json, format!("{:?}", adr.status)],
            )?;
            Ok(adr)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn get_adr_by_id(&self, id: &str) -> Result<Option<Adr>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let adr_id = id.to_string();
        
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("SELECT data FROM adrs WHERE adr_id = ?1")?;
            let mut rows = stmt.query_map([&adr_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;
            
            if let Some(row) = rows.next() {
                let data = row?;
                let adr: Adr = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                Ok(Some(adr))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn update_adr(&self, adr: Adr) -> Result<Adr> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let adr_json = serde_json::to_string(&adr)?;
        let adr_id = adr.id.clone();
        let status = format!("{:?}", adr.status);
        
        conn.interact(move |conn| {
            conn.execute(
                "UPDATE adrs SET data = ?1, status = ?2 WHERE adr_id = ?3",
                rusqlite::params![&adr_json, &status, &adr_id],
            )?;
            Ok(adr)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn delete_adr(&self, id: &str) -> Result<()> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let adr_id = id.to_string();
        
        conn.interact(move |conn| {
            conn.execute(
                "DELETE FROM adrs WHERE adr_id = ?1",
                rusqlite::params![&adr_id],
            )?;
            Ok(())
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn list_adrs(&self) -> Result<Vec<Adr>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("SELECT data FROM adrs")?;
            let adr_iter = stmt.query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;
            
            let mut adrs = Vec::new();
            for adr_result in adr_iter {
                let data = adr_result?;
                let adr: Adr = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                adrs.push(adr);
            }
            
            Ok(adrs)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }
}