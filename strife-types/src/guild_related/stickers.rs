use serde::{Deserialize, Serialize};

use crate::{Snowflake, user::User};

#[derive(Debug, Serialize, Deserialize)]
pub struct Sticker {
    id: Snowflake,
    pack_id: Option<Snowflake>,
    name: String,
    description: Option<String>,
    tags: String,
    asset: Option<String>,
    #[serde(rename = "type")]
    sticker_type: i32,
    format_type: i32,
    available: Option<bool>,
    guild_id: Option<Snowflake>,
    user: Option<User>,
    sort_value: i32,
}