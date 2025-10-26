mod common;

use ray_dashboard_client::jobs;

#[tokio::test]
async fn test_list_jobs() {
    let client = jobs::Client::new_with_user_agent(common::RAY_DASHBOARD_URL);
    let response = client.list_jobs().await.unwrap();
    for details in response.into_inner().iter() {
        println!("{:?}", details);
    }
}
