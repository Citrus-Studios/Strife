use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Op10 {
    op: i32,
    pub d: Op10Data,
    s: Option<i32>,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Op10Data {
    pub heartbeat_interval: i32,
    _trace: Option<Vec<String>>,
}