mod common;

use ray_dashboard_sdk::RayDashboardClient;

#[tokio::test]
async fn test_ping() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();
    client.ping().await.expect("Able to ping Ray dashboard");
}

#[tokio::test]
async fn test_get_version() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();
    let version = client.get_version().await.expect("Able to get version");
    assert_eq!(version.ray_version, "2.50.1");
}
