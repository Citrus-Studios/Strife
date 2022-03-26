use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleTags {
    bot_id: Option<Snowflake>,
    integration_id: Option<Snowflake>,
    premium_subscriber: Option<()>
}