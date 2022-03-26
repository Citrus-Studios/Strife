use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientStatus {
    desktop: Option<String>,
    mobile: Option<String>,
    web: Option<String>
}