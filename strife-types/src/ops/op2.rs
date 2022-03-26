use serde::{Serialize, Deserialize};

use crate::properties::Properties;

#[derive(Debug, Serialize, Deserialize)]
pub struct Op2 {
    pub op: i32,
    pub d: Option<Op2Data>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Op2Data {
    pub token: String,
    pub properities: Properties,
    pub compress: Option<bool>,
    pub large_threshold: Option<i32>,
    pub shards: Option<Vec<i32>>,
    pub intents: i32,
}