// llmdoc/src/services/user_story_service.rs

use crate::core::errors::Result;
use async_trait::async_trait;
use std::sync::Arc;
use crate::core::database::DbConnection;
use crate::core::models::user_story::UserStory;

#[async_trait]
pub trait UserStoryServiceTrait: Send + Sync {
    fn new(db_connection: Arc<DbConnection>) -> Self;
    async fn create_user_story(&self, user_story: UserStory) -> Result<UserStory>;
    async fn get_user_story_by_id(&self, id: &str) -> Result<Option<UserStory>>;
    async fn update_user_story(&self, user_story: UserStory) -> Result<UserStory>;
    async fn delete_user_story(&self, id: &str) -> Result<()>;
    async fn list_user_stories(&self) -> Result<Vec<UserStory>>;
}

#[derive(Clone)]
pub struct UserStoryService {
    db_connection: Arc<DbConnection>,
}

#[async_trait]
impl UserStoryServiceTrait for UserStoryService {
    fn new(db_connection: Arc<DbConnection>) -> Self {
        Self { db_connection }
    }

    async fn create_user_story(&self, user_story: UserStory) -> Result<UserStory> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let user_story_json = serde_json::to_string(&user_story)?;
        
        conn.interact(move |conn| {
            conn.execute(
                "INSERT INTO user_stories (story_id, data) VALUES (?1, ?2)",
                rusqlite::params![&user_story.id, &user_story_json],
            )?;
            Ok(user_story)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn get_user_story_by_id(&self, id: &str) -> Result<Option<UserStory>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let story_id = id.to_string();
        
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("SELECT data FROM user_stories WHERE story_id = ?1")?;
            let mut rows = stmt.query_map([&story_id], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;
            
            if let Some(row) = rows.next() {
                let data = row?;
                let user_story: UserStory = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                Ok(Some(user_story))
            } else {
                Ok(None)
            }
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn update_user_story(&self, user_story: UserStory) -> Result<UserStory> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let user_story_json = serde_json::to_string(&user_story)?;
        let story_id = user_story.id.clone();
        
        conn.interact(move |conn| {
            conn.execute(
                "UPDATE user_stories SET data = ?1 WHERE story_id = ?2",
                rusqlite::params![&user_story_json, &story_id],
            )?;
            Ok(user_story)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn delete_user_story(&self, id: &str) -> Result<()> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        let story_id = id.to_string();
        
        conn.interact(move |conn| {
            conn.execute(
                "DELETE FROM user_stories WHERE story_id = ?1",
                rusqlite::params![&story_id],
            )?;
            Ok(())
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }

    async fn list_user_stories(&self) -> Result<Vec<UserStory>> {
        let conn = self.db_connection.pool.get().await
            .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Failed to get DB connection: {}", e)))?;
        
        conn.interact(move |conn| {
            let mut stmt = conn.prepare("SELECT data FROM user_stories")?;
            let story_iter = stmt.query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(data)
            })?;
            
            let mut user_stories = Vec::new();
            for story_result in story_iter {
                let data = story_result?;
                let user_story: UserStory = serde_json::from_str(&data)
                    .map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
                user_stories.push(user_story);
            }
            
            Ok(user_stories)
        })
        .await
        .map_err(|e| crate::core::errors::Error::DatabaseOperation(format!("Interact error: {}", e)))?
        .map_err(|e: rusqlite::Error| crate::core::errors::Error::DatabaseOperation(format!("SQL execution error: {}", e)))
    }
}