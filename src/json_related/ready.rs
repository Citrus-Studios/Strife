use serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Ready {
    v: i32,
    user: ,
    guilds: ,
    session_id: String,
    shard: Option<Vec<i32>>,
    application: ,
}