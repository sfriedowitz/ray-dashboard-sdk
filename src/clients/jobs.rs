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
        let client = reqwest::Client::builder().build().unwrap();
        Self::new_with_client(base_url, client)
    }

    pub fn new_with_client(base_url: &str, client: reqwest::Client) -> crate::Result<Self> {
        let base_url = url::Url::parse(base_url)?;
        Ok(Self { base_url, client })
    }

    fn base_request(&self, method: reqwest::Method, path: &str) -> crate::Result<reqwest::RequestBuilder> {
        let url = self.base_url.join(path)?;
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
        let response = request.send().await?;
        Ok(response.json::<JobVersionResponse>().await?)
    }

    pub async fn submit_job(&self, payload: JobSubmitRequest) -> crate::Result<JobSubmitResponse> {
        let path = "/api/jobs";
        let request = self.base_request(reqwest::Method::POST, path)?;
        let response = request.json(&payload).send().await?;
        Ok(response.json::<JobSubmitResponse>().await?)
    }

    pub async fn list_jobs(&self) -> crate::error::Result<Vec<JobDetails>> {
        let path = "/api/jobs";
        let request = self.base_request(reqwest::Method::GET, path)?;
        let response = request.send().await?;
        Ok(response.json::<Vec<JobDetails>>().await?)
    }

    pub async fn get_job_details(&self, submission_id: &str) -> crate::Result<JobDetails> {
        let path = format!("/api/jobs/{}", submission_id);
        let request = self.base_request(reqwest::Method::GET, &path)?;
        let response = request.send().await?;
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
        let response = request.send().await?;
        Ok(response.json::<JobDeleteResponse>().await?)
    }

    pub async fn stop_job(&self, submission_id: &str) -> crate::Result<JobStopResponse> {
        let path = format!("/api/jobs/{}/stop", submission_id);
        let request = self.base_request(reqwest::Method::POST, &path)?;
        let response = request.send().await?;
        Ok(response.json::<JobStopResponse>().await?)
    }

    pub async fn get_job_logs(&self, submission_id: &str) -> crate::Result<JobLogsResponse> {
        let path = format!("/api/jobs/{}/logs", submission_id);
        let request = self.base_request(reqwest::Method::GET, &path)?;
        let response = request.send().await?;
        Ok(response.json::<JobLogsResponse>().await?)
    }
}
