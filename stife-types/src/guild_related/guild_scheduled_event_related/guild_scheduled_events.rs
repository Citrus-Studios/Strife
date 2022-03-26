use serde::{Deserialize, Serialize};

use crate::{Snowflake, Timestamp, user::User};

use super::guild_scheduled_metadata::GuildScheduledMetadata; 

#[derive(Debug, Serialize, Deserialize)]
pub struct GuildScheduledEvent {
    id: Snowflake,
    guild_id: Snowflake,
    channel_id: Option<Snowflake>,
    creator_id: Option<Snowflake>,
    name: String,
    description: Option<String>,
    scheduled_start_time: Timestamp,
    scheduled_end_time: Option<Timestamp>,
    privacy_level: i32,
    status: i32,
    entity_type: i32,
    entity_id: Option<Snowflake>,
    entity_metadata: Option<GuildScheduledMetadata>,
    creator: Option<User>,
    user_count: i32,
    image: Option<String>,
}