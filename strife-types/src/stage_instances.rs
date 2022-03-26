use serde::{Deserialize, Serialize};

use crate::Snowflake; 

#[derive(Debug, Serialize, Deserialize)]
pub struct StageInstance {
    id: Snowflake,
    guild_id: Snowflake,
    channel_id: Snowflake,
    topic: String,
    privacy_level: i32,
    discoverable_disabled: bool,
    guild_scheduled_event_id: Option<Snowflake>,
}