use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Secret {
    join: Option<String>,
    spectate: Option<String>,
    #[serde(rename = "match")]
    activity_match: Option<String>,
}