// llmdoc/src/core/models/sprint.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::core::models::task::Task; // Assuming tasks are part of a sprint

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub enum SprintStatus {
    Planning,
    Active,
    Completed,
    Cancelled,
}

impl std::fmt::Display for SprintStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for SprintStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Planning" => Ok(SprintStatus::Planning),
            "Active" => Ok(SprintStatus::Active),
            "Completed" => Ok(SprintStatus::Completed),
            "Cancelled" => Ok(SprintStatus::Cancelled),
            _ => Err(anyhow::anyhow!("Invalid SprintStatus: {}", s)),
        }
    }
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, Default)]
pub struct Retrospective {
    pub what_went_well: Vec<String>,
    pub what_could_improve: Vec<String>,
    pub action_items: Vec<String>,
    pub notes: Option<String>,
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone)]
pub struct Sprint {
    pub id: String, // UUID
    pub name: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub goals: Vec<String>,
    pub tasks: Vec<Task>, // Assuming tasks are linked to a sprint
    pub status: SprintStatus,
    pub velocity: Option<f32>,
    pub capacity: Option<f32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub retrospective: Option<Retrospective>,
}

impl Sprint {
    pub fn new(name: String, start_date: DateTime<Utc>, end_date: DateTime<Utc>) -> Self {
        Self {
            id: format!("sprint-{}", Uuid::new_v4().to_string()),
            name,
            description: None,
            start_date,
            end_date,
            goals: Vec::new(),
            tasks: Vec::new(),
            status: SprintStatus::Planning,
            velocity: None,
            capacity: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            retrospective: None,
        }
    }
}
