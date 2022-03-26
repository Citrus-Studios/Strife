use serde::{Deserialize, Serialize}; 

#[derive(Debug, Serialize, Deserialize)]
pub struct Properties {
    #[serde(rename = "$os")]
    pub os: String,
    #[serde(rename = "$browser")]
    pub browser: String,
    #[serde(rename = "$device")]
    pub device: String,
}