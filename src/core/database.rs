// llmdoc/src/core/database.rs

// Placeholder for database connection and operation logic
use crate::core::errors::{Error, Result};
use deadpool_sqlite::{Pool, Runtime};
use std::path::Path;

/// Represents the database connection pool.
#[derive(Clone)]
pub struct DbConnection {
    pub pool: Pool,
}

impl DbConnection {
    /// Creates a new database connection pool.
    pub async fn new(database_url: &str) -> Result<Self> {
        let cfg = deadpool_sqlite::Config::new(database_url.to_string());
        let pool = cfg
            .builder(Runtime::Tokio1)
            .map_err(|e| Error::DatabaseOperation(format!("Failed to build SQLite pool builder: {}", e)))?
            .build()
            .map_err(|e| Error::DatabaseOperation(format!("Failed to build deadpool_sqlite pool: {}", e)))?;

        // Test the connection
        let _conn = pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get connection from pool during initialization: {}", e)))?;
        tracing::info!("Database connection pool initialized successfully.");

        Ok(DbConnection { pool })
    }

    /// Runs database migrations.
    pub async fn run_migrations(&self) -> Result<()> {
        let conn = self.pool.get().await
            .map_err(|e| Error::DatabaseOperation(format!("Failed to get connection for migrations: {}", e)))?;
        conn.interact(|conn| {
            // Example migration: Create a simple table if it doesn't exist
            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS adr (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    status TEXT NOT NULL,
                    date TEXT NOT NULL,
                    deciders TEXT,
                    consulted TEXT,
                    informed TEXT,
                    decision TEXT NOT NULL,
                    alternatives TEXT,
                    consequences TEXT
                );
                CREATE TABLE IF NOT EXISTS sprints (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    start_date TEXT,
                    end_date TEXT,
                    goal TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL
                );
                CREATE TABLE IF NOT EXISTS tasks (
                    id TEXT PRIMARY KEY,
                    title TEXT NOT NULL,
                    description TEXT,
                    status TEXT NOT NULL,
                    task_type TEXT NOT NULL,
                    priority TEXT NOT NULL,
                    sprint_id TEXT,
                    assignee TEXT,
                    story_points INTEGER,
                    labels TEXT,
                    dependencies TEXT,
                    created_at TEXT NOT NULL,
                    updated_at TEXT NOT NULL,
                    created_by TEXT,
                    updated_by TEXT,
                    completed_at TEXT
                );"
            )?;
            Ok::<(), crate::core::errors::Error>(())
        })
        .await
        .map_err(|e| Error::DatabaseOperation(format!("Deadpool interact error during migration: {:?}", e)))?
        .map_err(|e| Error::DatabaseOperation(format!("Migration execution failed within interact block: {}", e)))?;

        tracing::info!("Database migrations executed successfully.");
        Ok(())
    }
}


/// Ensures the database file's parent directory exists.
pub fn ensure_db_parent_dir_exists(database_url: &str) -> Result<()> {
    let db_path = Path::new(database_url);
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)
                .map_err(|e| Error::IoError(e))?;
            tracing::info!("Created database directory: {:?}", parent);
        }
    }
    Ok(())
}