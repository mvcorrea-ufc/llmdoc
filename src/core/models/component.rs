// llmdoc/src/core/models/component.rs

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub enum ComponentType {
    Module,
    Service,
    Library,
    Database,
    Api,
    Other,
}

impl std::fmt::Display for ComponentType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for ComponentType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "module" => Ok(ComponentType::Module),
            "service" => Ok(ComponentType::Service),
            "library" => Ok(ComponentType::Library),
            "database" => Ok(ComponentType::Database),
            "api" => Ok(ComponentType::Api),
            "other" => Ok(ComponentType::Other),
            _ => Err(anyhow::anyhow!("Invalid ComponentType: {}", s)),
        }
    }
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone)]
pub struct Component {
    pub id: String, // comp-uuid
    pub name: String,
    pub component_type: ComponentType,
    pub description: String,
    pub dependencies: Vec<String>, // IDs of other components
    pub interfaces: Vec<String>,   // e.g., API endpoints, message queues
    pub tech_stack: Vec<String>,   // e.g., Rust, Tokio, Postgres
    pub owner: Option<String>,     // Team or individual
    pub documentation_url: Option<String>,
    pub repository_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, String>, // Flexible metadata
}

impl Component {
    pub fn new(name: String, component_type: ComponentType, description: String) -> Self {
        Self {
            id: format!("comp-{}", Uuid::new_v4().to_string()),
            name,
            component_type,
            description,
            dependencies: Vec::new(),
            interfaces: Vec::new(),
            tech_stack: Vec::new(),
            owner: None,
            documentation_url: None,
            repository_url: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            metadata: HashMap::new(),
        }
    }
}