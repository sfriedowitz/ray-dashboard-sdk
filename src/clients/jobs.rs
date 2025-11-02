use std::time::Duration;

use tokio::time::{Instant, sleep};
use tracing::info;

use crate::schemas::jobs::{
    JobDeleteResponse, JobDetails, JobLogsResponse, JobStatus, JobStopResponse, JobSubmitRequest,
    JobSubmitResponse, JobVersionResponse,
};

#[derive(Debug, Clone)]
pub struct JobSubmissionClient {
    base_url: url::Url,
    client: reqwest::Client,
}

// Constructors
impl JobSubmissionClient {
    pub fn new(base_url: &str) -> crate::Result<Self> {
        let client = reqwest::Client::builder().build()?;
        Self::new_with_client(base_url, client)
    }

    pub fn new_with_client(base_url: &str, client: reqwest::Client) -> crate::Result<Self> {
        let base_url = url::Url::parse(base_url)?;
        Ok(Self { base_url, client })
    }

    /// Build base request with common headers.
    /// Ray dashboard server requires User-Agent header to be set or else 500s.
    fn base_request(&self, method: reqwest::Method, path: &str) -> crate::Result<reqwest::RequestBuilder> {
        let url = self.base_url.join(path)?;
        info!("Building request: {} {}", method, url);
        let request = self
            .client
            .request(method, url)
            .header("User-Agent", crate::constants::SDK_USER_AGENT);
        Ok(request)
    }
}

// API
impl JobSubmissionClient {
    pub async fn ping(&self) -> crate::Result<()> {
        self.get_version().await?;
        Ok(())
    }

    pub async fn get_version(&self) -> crate::Result<JobVersionResponse> {
        let path = "/api/version";
        let request = self.base_request(reqwest::Method::GET, path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobVersionResponse>().await?)
    }

    pub async fn submit_job(&self, payload: &JobSubmitRequest) -> crate::Result<JobSubmitResponse> {
        let path = "/api/jobs/";
        let request = self.base_request(reqwest::Method::POST, path)?;
        let response = request.json(payload).send().await?.error_for_status()?;
        Ok(response.json::<JobSubmitResponse>().await?)
    }

    pub async fn list_jobs(&self) -> crate::error::Result<Vec<JobDetails>> {
        let path = "/api/jobs/";
        let request = self.base_request(reqwest::Method::GET, path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<Vec<JobDetails>>().await?)
    }

    pub async fn get_job_details(&self, submission_id: &str) -> crate::Result<JobDetails> {
        let path = format!("/api/jobs/{}", submission_id);
        let request = self.base_request(reqwest::Method::GET, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobDetails>().await?)
    }

    /// Get job status by fetching details and then extracting status
    pub async fn get_job_status(&self, submission_id: &str) -> crate::Result<JobStatus> {
        let details = self.get_job_details(submission_id).await?;
        Ok(details.status)
    }

    pub async fn delete_job(&self, submission_id: &str) -> crate::Result<JobDeleteResponse> {
        let path = format!("/api/jobs/{}", submission_id);
        let request = self.base_request(reqwest::Method::DELETE, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobDeleteResponse>().await?)
    }

    pub async fn stop_job(&self, submission_id: &str) -> crate::Result<JobStopResponse> {
        let path = format!("/api/jobs/{}/stop", submission_id);
        let request = self.base_request(reqwest::Method::POST, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobStopResponse>().await?)
    }

    pub async fn get_job_logs(&self, submission_id: &str) -> crate::Result<JobLogsResponse> {
        let path = format!("/api/jobs/{}/logs", submission_id);
        let request = self.base_request(reqwest::Method::GET, &path)?;
        let response = request.send().await?.error_for_status()?;
        Ok(response.json::<JobLogsResponse>().await?)
    }

    /// Wait for the job to reach a terminal state
    /// Return an error if the job does not reach a terminal state within the provided max duration.
    pub async fn wait_for_terminal<D: Into<std::time::Duration>>(
        &self,
        submission_id: &str,
        max_duration: Option<D>,
    ) -> crate::Result<()> {
        let start = Instant::now();
        let max_duration = max_duration.map(|d| d.into());

        loop {
            let status = self.get_job_status(submission_id).await?;
            if status.is_terminal() {
                return Ok(());
            }

            if let Some(max_duration) = max_duration {
                if start.elapsed() >= max_duration {
                    return Err(crate::Error::Generic(format!(
                        "Job {} did not reach terminal state within {:?}",
                        submission_id, max_duration
                    )));
                }
            }

            sleep(Duration::from_millis(500)).await;
        }
    }
}
