use serde::{Deserialize, Serialize};

use super::user::User; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Ready {
    v: i32,
    user: User,
    guilds: ,
    session_id: String,
    shard: Option<Vec<i32>>,
    application: ,
}