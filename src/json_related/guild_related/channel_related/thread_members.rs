use serde::{Deserialize, Serialize};

use crate::{Snowflake, Timestamp};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMember {
    id: Option<Snowflake>,
    user_id: Option<Snowflake>,
    join_timestamp: Timestamp,
    flags: i32,
}