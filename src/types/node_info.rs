use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub version: String,
    #[serde(rename = "block_number")]
    pub block_number: i64,
    #[serde(rename = "chain_id")]
    // TOOD: This should be a `Chain`, probably need to do manual deserialization
    pub chain_id: i64,
    pub chain: String,
    pub syncing: bool,
}
