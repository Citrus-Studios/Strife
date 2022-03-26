use serde::{Deserialize, Serialize};

use crate::Snowflake;

use super::{user::User, client_statuses::ClientStatus, activities_related::activities::Activity};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceUpdate {
    user: User,
    guild_id: Snowflake,
    status: String,
    activites: Vec<Activity>,
    client_status: ClientStatus
}