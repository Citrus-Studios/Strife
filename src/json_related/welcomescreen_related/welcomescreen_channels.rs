use serde::{Deserialize, Serialize};

use crate::Snowflake;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WelcomeScreenChannel {
    channel_id: Snowflake,
    description: String,
    emoji_id: Option<Snowflake>,
    emoji_name: Option<String>,
}