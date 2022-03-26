use serde::{Deserialize, Serialize};

use crate::Snowflake;

use super::role_tags::RoleTags;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    id: Snowflake,
    name: String,
    color: i32,
    hoist: bool,
    icon: Option<String>,
    unicode_emoji: Option<String>,
    position: i32,
    permissions: String,
    managed: bool,
    mentionable: bool,
    tags: Option<RoleTags>
}