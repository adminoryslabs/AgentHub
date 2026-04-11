use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    #[serde(rename = "env")]
    pub environment: String,
    pub preferred_editor: String,
    pub default_agent: String,
    #[serde(default)]
    pub tags: Vec<String>,
    pub last_opened_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        name: String,
        path: String,
        environment: String,
        preferred_editor: String,
        default_agent: String,
        tags: Vec<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            path,
            environment,
            preferred_editor,
            default_agent,
            tags,
            last_opened_at: None,
            created_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsStore {
    pub version: u32,
    pub projects: Vec<Project>,
}

impl ProjectsStore {
    pub fn new() -> Self {
        Self {
            version: 1,
            projects: Vec::new(),
        }
    }
}

impl Default for ProjectsStore {
    fn default() -> Self {
        Self::new()
    }
}
