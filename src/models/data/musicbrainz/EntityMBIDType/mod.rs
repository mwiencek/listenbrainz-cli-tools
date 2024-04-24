use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[serde(tag = "type")]
pub enum EntityMBIDType {
    Artist(String),
    Recording(String),
    Release(String),
}

