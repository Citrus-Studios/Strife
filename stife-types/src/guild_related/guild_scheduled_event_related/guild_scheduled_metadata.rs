use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildScheduledMetadata {
    location: Option<String>
}