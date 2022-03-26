use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityTimestamp {
    start: Option<i32>,
    end: Option<i32>
}