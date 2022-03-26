use serde::{Deserialize, Serialize};

use crate::Timestamp;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMetadata {
    archived: bool,
    auto_archive_duration: i32,
    archive_timestamp: Timestamp,
    locked: bool,
    invitable: Option<bool>,
    create_timestamp: Option<Timestamp>
}