// llmdoc/src/core/models.rs

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// Placeholder for data models (Task, Sprint, Component, etc.)

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: String, // UUID
    pub title: String,
    pub description: Option<String>,
    pub status: String, // e.g., "Todo", "InProgress", "Done"
    pub priority: Option<String>, // e.g., "High", "Medium", "Low"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub sprint_id: Option<String>, // Foreign key to Sprint
    pub component_id: Option<String>, // Foreign key to Component
    // pub assigned_to: Option<String>, // User ID
    // pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Sprint {
    pub id: String, // UUID
    pub name: String,
    pub start_date: Option<DateTime<Utc>>,
    pub end_date: Option<DateTime<Utc>>,
    pub goal: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Add other models like Component, ADR, UserStory as needed

pub fn models_init_message() {
    tracing::debug!("Models module initialized (placeholder).");
}