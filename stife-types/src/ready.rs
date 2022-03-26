use serde::{Deserialize, Serialize};

use super::{user::User, guild_related::guild::Guild}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Ready {
    v: i32,
    user: User,
    guilds: Vec<Guild>,
    session_id: String,
    shard: Option<Vec<i32>>,
    application: Vec<Application>,
}