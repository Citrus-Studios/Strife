use serde::{Deserialize, Serialize};

use crate::{Snowflake, user::User};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Emoji {
    id: Option<Snowflake>,
    name: Option<String>,
    roles: Option<Vec<i32>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}