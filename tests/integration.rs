use safe_rs::client::SafeClient;

#[tokio::test]
async fn it_gets_service_info() {
    let client = SafeClient::new();
    let service_info = client.service_info().await.unwrap();
    assert_eq!(service_info.name, "Safe Transaction Service");
}

#[tokio::test]
async fn it_gets_node_info() {
    let client = SafeClient::new();
    let node_info = client.ethereum_rpc_info().await.unwrap();
    assert!(node_info.version.contains("erigon") || node_info.version.contains("geth"));
}
