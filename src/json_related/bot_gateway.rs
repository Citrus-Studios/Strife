use serde::{Deserialize, Serialize};

use super::session_start_limit::SessionStartLimit; 

// Bot Gateway JSON
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotGateway {
    pub url: String,
    pub shards: i32,
    pub session_start_limit: SessionStartLimit,
}