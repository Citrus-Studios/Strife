use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Op1 {
    pub op: i32,
    pub d: Option<i32>,
    pub s: Option<i32>,
}