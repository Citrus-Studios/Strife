use serde::{Deserialize, Serialize};

use crate::Snowflake; 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: Snowflake,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    banner: Option<String>,
    accent_color: Option<i32>,
    locale: Option<String>,
    verified: Option<i32>,
    email: Option<String>,
    flags: Option<i32>,
    premium_type: Option<i32>,
    public_flags: Option<i32>
}