use serde::{Deserialize, Serialize}; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStartLimit {
    pub total: i32,
    pub remaining: i32,
    pub reset_after: i32,
    pub max_concurrency: i32,
}