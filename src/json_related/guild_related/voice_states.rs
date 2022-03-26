use serde::{Deserialize, Serialize};

use crate::{Snowflake, Timestamp};

use super::guild_members::GuildMember;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceState {
    guild_id: Option<Snowflake>,
    channel_id: Option<Snowflake>,
    user_id: Snowflake,
    member: Option<GuildMember>,
    session_id: String,
    deaf: bool,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: Option<bool>,
    self_video: Option<bool>,
    suppress: bool,
    request_to_speak_timestamp: Option<Timestamp>
}