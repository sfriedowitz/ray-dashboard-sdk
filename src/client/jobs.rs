use std::time::{Duration, Instant};

use async_trait::async_trait;
use futures_timer::Delay;

use crate::{
    RayDashboardClient,
    schemas::jobs::{
        JobDeleteResponse, JobDetails, JobLogsResponse, JobStatus, JobStopResponse, JobSubmitRequest,
        JobSubmitResponse,
    },
};

#[async_trait]
pub trait JobSubmissionAPI {
    /// Submit a new job
    async fn submit_job(&self, payload: &JobSubmitRequest) -> crate::Result<JobSubmitResponse>;

    /// List all jobs
    async fn list_jobs(&self) -> crate::error::Result<Vec<JobDetails>>;

    /// Get job details by submission ID
    async fn get_job_details(&self, submission_id: &str) -> crate::Result<JobDetails>;

    /// Get job status by fetching details and then extracting status
    async fn get_job_status(&self, submission_id: &str) -> crate::Result<JobStatus>;

    /// Delete a job
    async fn delete_job(&self, submission_id: &str) -> crate::Result<JobDeleteResponse>;

    /// Stop a running job
    async fn stop_job(&self, submission_id: &str) -> crate::Result<JobStopResponse>;

    /// Get the logs for a job
    async fn get_job_logs(&self, submission_id: &str) -> crate::Result<JobLogsResponse>;

    /// Wait for the job to reach a terminal state
    /// Return an error if the job does not reach a terminal state within the provided max duration.
    async fn wait_for_terminal(
        &self,
        submission_id: &str,
        max_duration: Option<Duration>,
    ) -> crate::Result<()>;
}

#[async_trait]
impl JobSubmissionAPI for RayDashboardClient {
    async fn submit_job(&self, payload: &JobSubmitRequest) -> crate::Result<JobSubmitResponse> {
        let path = "/api/jobs/";
        let request = self.base_request(reqwest::Method::POST, path)?;
        let response = request.json(payload).send().await?.error_for_status()?;
        Ok(response.json::<JobSubmitResponse>().await?)
    }

    async fn list_jobs(&self) -> crate::error::Result<Vec<JobDetails>> {
        let path = "/api/jobs/";
        let request = self.base_request(reqwest::Method::GET, path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<Vec<JobDetails>>().await?)
    }

    async fn get_job_details(&self, submission_id: &str) -> crate::Result<JobDetails> {
        let path = format!("/api/jobs/{}", submission_id);
        let request = self.base_request(reqwest::Method::GET, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobDetails>().await?)
    }

    async fn get_job_status(&self, submission_id: &str) -> crate::Result<JobStatus> {
        let details = self.get_job_details(submission_id).await?;
        Ok(details.status)
    }

    async fn delete_job(&self, submission_id: &str) -> crate::Result<JobDeleteResponse> {
        let path = format!("/api/jobs/{}", submission_id);
        let request = self.base_request(reqwest::Method::DELETE, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobDeleteResponse>().await?)
    }

    async fn stop_job(&self, submission_id: &str) -> crate::Result<JobStopResponse> {
        let path = format!("/api/jobs/{}/stop", submission_id);
        let request = self.base_request(reqwest::Method::POST, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobStopResponse>().await?)
    }

    async fn get_job_logs(&self, submission_id: &str) -> crate::Result<JobLogsResponse> {
        let path = format!("/api/jobs/{}/logs", submission_id);
        let request = self.base_request(reqwest::Method::GET, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobLogsResponse>().await?)
    }

    async fn wait_for_terminal(
        &self,
        submission_id: &str,
        max_duration: Option<Duration>,
    ) -> crate::Result<()> {
        let start = Instant::now();

        loop {
            let status = self.get_job_status(submission_id).await?;
            if status.is_terminal() {
                return Ok(());
            }

            if let Some(max_duration) = max_duration
                && start.elapsed() >= max_duration
            {
                return Err(crate::Error::Generic(format!(
                    "Job {} did not reach terminal state within {:?}",
                    submission_id, max_duration
                )));
            }

            Delay::new(Duration::from_millis(500)).await;
        }
    }
}
