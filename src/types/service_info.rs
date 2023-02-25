use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    #[serde(rename = "api_version")]
    pub api_version: String,
    pub secure: bool,
    pub host: String,
    pub headers: Vec<String>,
    #[serde(flatten)]
    pub settings: Settings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    #[serde(rename = "AWS_CONFIGURED")]
    pub aws_configured: Option<bool>,
    #[serde(rename = "AWS_S3_BUCKET_NAME")]
    pub aws_s3_bucket_name: Option<String>,
    #[serde(rename = "AWS_S3_PUBLIC_URL")]
    pub aws_s3_public_url: Option<String>,
    #[serde(rename = "ETHEREUM_NODE_URL")]
    pub ethereum_node_url: Option<String>,
    #[serde(rename = "ETHEREUM_TRACING_NODE_URL")]
    pub ethereum_tracing_node_url: Option<String>,
    #[serde(rename = "ETH_EVENTS_BLOCK_PROCESS_LIMIT")]
    pub eth_events_block_process_limit: Option<i64>,
    #[serde(rename = "ETH_EVENTS_BLOCK_PROCESS_LIMIT_MAX")]
    pub eth_events_block_process_limit_max: Option<i64>,
    #[serde(rename = "ETH_EVENTS_QUERY_CHUNK_SIZE")]
    pub eth_events_query_chunk_size: Option<i64>,
    #[serde(rename = "ETH_EVENTS_UPDATED_BLOCK_BEHIND")]
    pub eth_events_updated_block_behind: Option<i64>,
    #[serde(rename = "ETH_INTERNAL_NO_FILTER")]
    pub eth_internal_no_filter: Option<bool>,
    #[serde(rename = "ETH_INTERNAL_TRACE_TXS_BATCH_SIZE")]
    pub eth_internal_trace_txs_batch_size: Option<i64>,
    #[serde(rename = "ETH_INTERNAL_TXS_BLOCK_PROCESS_LIMIT")]
    pub eth_internal_txs_block_process_limit: Option<i64>,
    #[serde(rename = "ETH_L2_NETWORK")]
    pub eth_l2_network: Option<bool>,
    #[serde(rename = "ETH_REORG_BLOCKS")]
    pub eth_reorg_blocks: Option<i64>,
    #[serde(rename = "SSO_ENABLED")]
    pub sso_enabled: Option<bool>,
    #[serde(rename = "TOKENS_LOGO_BASE_URI")]
    pub tokens_logo_base_uri: Option<String>,
    #[serde(rename = "TOKENS_LOGO_EXTENSION")]
    pub tokens_logo_extension: Option<String>,
}
