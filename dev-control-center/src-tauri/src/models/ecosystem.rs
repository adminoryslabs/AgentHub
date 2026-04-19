use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ecosystem {
    pub id: Uuid,
    pub name: String,
    pub root_path: String,
    #[serde(rename = "env")]
    pub environment: String,
    pub default_agent: String,
    pub created_at: DateTime<Utc>,
}

impl Ecosystem {
    pub fn new(
        name: String,
        root_path: String,
        environment: String,
        default_agent: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            root_path,
            environment,
            default_agent,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemsStore {
    pub version: u32,
    pub ecosystems: Vec<Ecosystem>,
}

impl EcosystemsStore {
    pub fn new() -> Self {
        Self {
            version: 1,
            ecosystems: Vec::new(),
        }
    }
}

impl Default for EcosystemsStore {
    fn default() -> Self {
        Self::new()
    }
}
