// llmdoc/src/core/models/user_story.rs

use chrono::{DateTime, Utc};

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone)]
pub struct UserStory {
    pub id: String, // US-uuid
    pub title: String, // e.g., "As a [user type], I want [goal] so that [reason]"
    pub persona: String, // "As a [persona]"
    pub want: String,    // "I want [want]"
    pub benefit: String, // "so that [benefit]"
    pub description: Option<String>,
    pub acceptance_criteria: Vec<String>,
    pub story_points: Option<u8>,
    pub priority: crate::core::models::task::Priority, // Using the Priority enum from task module
    pub epic_id: Option<String>, // Optional link to an Epic task
    pub tasks: Vec<String>, // IDs of linked tasks
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserStory {
    pub fn new(id: String, title: String, persona: String, want: String, benefit: String) -> Self {
        Self {
            id,
            title,
            persona,
            want,
            benefit,
            description: None,
            acceptance_criteria: Vec::new(),
            story_points: None,
            priority: crate::core::models::task::Priority::Medium, // Default priority
            epic_id: None,
            tasks: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}
