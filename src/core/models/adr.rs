// llmdoc/src/core/models/adr.rs

use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, PartialEq)]
pub enum AdrStatus {
    Proposed,
    Accepted,
    Rejected,
    Deprecated,
    Superseded,
}

impl std::fmt::Display for AdrStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::str::FromStr for AdrStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "proposed" => Ok(AdrStatus::Proposed),
            "accepted" => Ok(AdrStatus::Accepted),
            "rejected" => Ok(AdrStatus::Rejected),
            "deprecated" => Ok(AdrStatus::Deprecated),
            "superseded" => Ok(AdrStatus::Superseded),
            _ => Err(anyhow::anyhow!("Invalid AdrStatus: {}", s)),
        }
    }
}

#[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone)]
pub struct Adr {
    pub id: String, // ADR-uuid
    pub title: String,
    pub status: AdrStatus,
    pub context: String,
    pub decision: String,
    pub consequences: String,
    pub alternatives: Vec<String>, // List of alternative options considered
    pub related_adrs: Vec<String>, // IDs of related ADRs
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
}

impl Adr {
    pub fn new(title: String, context: String, decision: String, consequences: String) -> Self {
        Self {
            id: format!("adr-{}", Uuid::new_v4().to_string()),
            title,
            status: AdrStatus::Proposed, // Default status
            context,
            decision,
            consequences,
            alternatives: Vec::new(),
            related_adrs: Vec::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            approved_by: None,
            approved_at: None,
        }
    }
}