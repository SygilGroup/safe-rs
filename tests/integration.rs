use safe_rs::client::SafeClient;

#[tokio::test]
async fn it_gets_service_info() {
    let client = SafeClient::new();
    let service_info = client.service_info().await.unwrap();
    assert_eq!(service_info.name, "Safe Transaction Service");
}
