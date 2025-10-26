mod setup;

use reqwest;

#[tokio::test]
async fn test_cluster_info() {
    // setup::RayGuard::start();

    let response = reqwest::get("http://127.0.0.1:8265/api/jobs/")
        .await
        .unwrap();
    dbg!(response.text().await.unwrap());

    // let client = jobs::Client::new(setup::RAY_DASHBOARD_URL);

    // let resp = client.list_jobs().await.unwrap();
    // for details in resp.into_inner().into_iter() {
    //     dbg!(details);
    // }
}
