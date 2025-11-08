mod common;

use std::time::Duration;

use ray_dashboard_sdk::{
    JobSubmissionAPI, RayDashboardClient,
    schemas::jobs::{JobStatus, JobSubmitRequest},
};

fn random_submission_id() -> String {
    let id = uuid::Uuid::new_v4().to_string();
    format!("test-{}", id)
}

#[tokio::test]
async fn test_submit_job() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();

    let submission_id = random_submission_id();
    let payload = JobSubmitRequest::new("echo Hello, World!").with_submission_id(&submission_id);

    let response = client.submit_job(&payload).await.expect("Able to submit job");
    assert_eq!(response.submission_id, submission_id);

    let jobs = client.list_jobs().await.expect("Able to list jobs");
    assert!(!jobs.is_empty());
}

#[tokio::test]
async fn test_get_job() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();

    let entrypoint = "echo Hello, World!";
    let submission_id = random_submission_id();
    let payload = JobSubmitRequest::new(entrypoint)
        .with_submission_id(&submission_id)
        .with_metadata_item("environment", "dev");

    client.submit_job(&payload).await.expect("Able to submit job");

    let job_details = client
        .get_job_details(&submission_id)
        .await
        .expect("Able to get job details");

    assert_eq!(job_details.entrypoint, entrypoint);
    assert_eq!(job_details.submission_id, Some(submission_id));

    let job_metadata = job_details.metadata.as_ref().expect("Job has metadata");
    assert!(job_metadata.get("environment").is_some());
}

#[tokio::test]
async fn test_stop_and_delete_job() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();

    let submission_id = random_submission_id();
    let payload = JobSubmitRequest::new("sleep 60").with_submission_id(&submission_id);
    client.submit_job(&payload).await.expect("Able to submit job");

    client.stop_job(&submission_id).await.expect("Able to stop job");
    client
        .wait_for_terminal(&submission_id, Some(Duration::from_secs(5)))
        .await
        .unwrap();

    let status = client
        .get_job_status(&submission_id)
        .await
        .expect("Able to get job status");
    assert_eq!(status, JobStatus::STOPPED);

    client
        .delete_job(&submission_id)
        .await
        .expect("Able to delete job");
}

#[tokio::test]
async fn test_get_job_logs() {
    let client = RayDashboardClient::new(common::RAY_DASHBOARD_URL).unwrap();

    let submission_id = random_submission_id();
    let payload = JobSubmitRequest::new("echo 'ABC123'").with_submission_id(&submission_id);
    client.submit_job(&payload).await.expect("Able to submit job");

    // Wait for job to complete
    client
        .wait_for_terminal(&submission_id, Some(Duration::from_secs(5)))
        .await
        .unwrap();

    let logs = client
        .get_job_logs(&submission_id)
        .await
        .expect("Able to get job logs");
    let log_lines = logs.lines().collect::<Vec<&str>>();
    assert!(log_lines.iter().any(|line| line.contains("ABC123")));
}
