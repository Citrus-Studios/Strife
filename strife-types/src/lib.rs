pub mod bot_gateway;
pub mod properties;
pub mod ready;
pub mod session_start_limit;
pub mod user;
pub mod presences;
pub mod client_statuses;
pub mod stage_instances;

pub mod ops;
pub mod guild_related;
pub mod activities_related;
pub mod welcomescreen_related;

pub type Snowflake = u64;
pub type Timestamp = String;
