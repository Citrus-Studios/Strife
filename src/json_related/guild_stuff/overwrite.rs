use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Overwrite {
    id: Snowflake,
    #[serde(rename = "type")]
    overwrite_type: i32,
    allow: String,
    deny: String,
}