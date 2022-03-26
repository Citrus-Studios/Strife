use serde::{Deserialize, Serialize};

use crate::{Snowflake, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    avatar: Option<String>,
    roles: Vec<Snowflake>,
    joined_at: String,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool,
    pending: Option<bool>,
    permissions: Option<String>,
    communication_disabled_until: Option<String>
}