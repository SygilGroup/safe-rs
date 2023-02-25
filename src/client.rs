use crate::constants;
use crate::types::service_info::ServiceInfo;
use reqwest::Client;
use thiserror::Error;

pub struct SafeClient {
    client: Client,
}

#[derive(Error, Debug)]
pub enum SafeClientError {
    #[error("Unknown error")]
    UnknownError,
    #[error("Conversion error")]
    ConversionError,
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
            .get(format!("{}/{}", constants::SAFE_MAINNET_URL, "v1/about/"))
            .send()
            .await
            .map_err(|_| SafeClientError::UnknownError)?
            .json()
            .await
            .map_err(|_| SafeClientError::ConversionError)?;

        Ok(service_info)
    }
}
