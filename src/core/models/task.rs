// llmdoc/src/core/models/task.rs

use chrono::{DateTime, Utc};

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Blocked,
    Cancelled,
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub enum TaskType {
    Bug,
    Feature,
    Task,
    Epic,
    Story,
    Spike,
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone)]
pub struct Task {
    pub id: String, // UUID
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub task_type: TaskType,
    pub priority: Priority,
    pub sprint_id: Option<String>, // Foreign key to Sprint
    pub assignee: Option<String>, // User ID
    pub story_points: Option<u8>,
    pub labels: Vec<String>,
    pub dependencies: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub updated_by: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl Task {
    pub fn new(id: String, title: String, task_type: TaskType) -> Self {
        Self {
            id,
            title,
            description: None,
            status: TaskStatus::Todo,
            task_type,
            priority: Priority::Medium, // Default priority
            sprint_id: None,
            assignee: None,
            story_points: None,
            labels: Vec::new(),
            dependencies: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            updated_by: None,
            completed_at: None,
        }
    }

    pub fn set_status(&mut self, new_status: TaskStatus, updated_by: Option<String>) {
        self.status = new_status;
        self.updated_at = Utc::now();
        self.updated_by = updated_by;
        if self.status == TaskStatus::Done {
            self.completed_at = Some(Utc::now());
        } else {
            self.completed_at = None;
        }
    }
}