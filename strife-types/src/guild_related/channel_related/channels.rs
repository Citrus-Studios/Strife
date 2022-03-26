use serde::{Deserialize, Serialize};

use crate::{Snowflake, {user::User, guild_related::overwrite::Overwrite}, Timestamp};

use super::{thread_metadatas::ThreadMetadata, thread_members::ThreadMember};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    id: Snowflake,
    #[serde(rename = "type")]
    channel_type: i32,
    guild_id: Option<Snowflake>,
    position: i32,
    permission_overwrites: Option<Vec<Overwrite>>,
    name: Option<String>,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<Snowflake>,
    bitrate: Option<i32>,
    user_limit: Option<i32>,
    rate_limit_per_user: Option<i32>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<Snowflake>,
    application_id: Option<Snowflake>,
    parent_id: Option<Snowflake>,
    last_pin_timestamp: Option<Timestamp>,
    rtc_region: Option<String>,
    video_quality_mod: Option<i32>,
    message_count: Option<i32>,
    member_count: Option<i32>,
    thread_metadata: Option<ThreadMetadata>,
    member: Option<ThreadMember>,
    default_auto_archive_duration: i32,
    permissions: String,
} 