use serde::{Deserialize, Serialize}; 

// Bot Gateway JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotGateway {
    pub url: String,
    pub shards: i32,
    pub session_start_limit: SessionStartLimit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStartLimit {
    pub total: i32,
    pub remaining: i32,
    pub reset_after: i32,
    pub max_concurrency: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "$device")]
    pub device: String,
}