use crate::constants;
use crate::types::node_tracing_info::NodeTracingInfo;
use crate::types::{node_info::NodeInfo, routes::SafeRoute, service_info::ServiceInfo};
use reqwest::Client;

pub struct SafeClient {
    client: Client,
}

#[derive(Debug, thiserror::Error)]
pub enum SafeClientError {
    /// Reqwest error
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    /// Serde JSON error
    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),
    /// Other error, marked as unknown
    #[error("Unknown error")]
    UnknownError,
}

impl SafeClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    /// Retrieves the service info from Safe.
    pub async fn service_info(&self) -> Result<ServiceInfo, SafeClientError> {
        let service_info: ServiceInfo = self
            .client
            .get(format!(
                "{}/{}",
                constants::SAFE_MAINNET_URL,
                SafeRoute::ServiceInfo.to_string(),
            ))
            .send()
            .await?
            .json()
            .await?;

        Ok(service_info)
    }

    pub async fn ethereum_rpc_info(&self) -> Result<NodeInfo, SafeClientError> {
        let node_info: NodeInfo = self
            .client
            .get(format!(
                "{}/{}",
                constants::SAFE_MAINNET_URL,
                SafeRoute::EthereumRpcInfo.to_string(),
            ))
            .send()
            .await?
            .json()
            .await?;
        Ok(node_info)
    }

    pub async fn ethereum_tracing_info(&self) -> Result<NodeTracingInfo, SafeClientError> {
        let node_tracing_info: NodeTracingInfo = self
            .client
            .get(format!(
                "{}/{}",
                constants::SAFE_MAINNET_URL,
                SafeRoute::EthereumTracingRpcInfo.to_string(),
            ))
            .send()
            .await?
            .json()
            .await?;
        Ok(node_tracing_info)
    }
}
