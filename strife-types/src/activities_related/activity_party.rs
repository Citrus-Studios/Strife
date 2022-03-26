use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Party {
    id: Option<String>,
    size: Option<Vec<i32>>,
}