use serde::{Deserialize, Serialize};

use crate::{Snowflake, guild_related::emojis::Emoji};

use super::{activity_timestamps::ActivityTimestamp, activity_party::Party, activity_assets::Asset, activity_secrets::Secret, activity_buttons::Button};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    name: String,
    #[serde(rename = "type")]
    activity_type: u8,
    url: Option<String>,
    created_at: i32,
    timestamps: Option<ActivityTimestamp>,
    application_id: Option<Snowflake>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<Emoji>,
    party: Option<Party>,
    assets: Option<Asset>,
    secrets: Option<Secret>,
    instance: Option<bool>,
    flags: Option<i32>,
    buttons: Option<Vec<Button>>
}