use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
/// Metadata about an CDN asset
pub struct AssetMetadata {
    pub exists: bool,
    pub path: String,
    pub default_path: String,
    #[serde(rename = "type")]
    pub kind: String,
    pub size: u64,
    pub last_modified: Option<DateTime<Utc>>,
    pub errors: Vec<String>,
}
