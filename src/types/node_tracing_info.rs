use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeTracingInfo {
    pub version: String,
    pub block_number: i64,
    pub chain_id: i64,
    pub chain: String,
    pub syncing: bool,
}
