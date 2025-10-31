use crate::schemas::jobs::{
    JobDeleteResponse, JobDetails, JobLogsResponse, JobStatus, JobStopResponse, JobSubmitRequest,
    JobSubmitResponse,
};

#[derive(Debug, Clone)]
pub struct JobSubmissionClient {
    base_url: String,
    client: reqwest::Client,
}

// Constructor methods
impl JobSubmissionClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        let client = reqwest::Client::builder().build().unwrap();
        Self::new_with_client(base_url, client)
    }

    pub fn new_with_client(base_url: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            base_url: base_url.into(),
            client,
        }
    }

    fn build_request(&self, path: &str) -> crate::Result<reqwest::Request> {
        todo!()
    }
}

// API
impl JobSubmissionClient {
    pub async fn submit_job(&self, request: JobSubmitRequest) -> crate::Result<JobSubmitResponse> {
        let path = "/api/jobs";
        let url = format!("{}{}", self.base_url, path);
        todo!()
    }

    pub async fn list_jobs(&self) -> crate::error::Result<Vec<JobDetails>> {
        todo!()
    }

    pub async fn get_job_info(&self, submission_id: &str) -> crate::Result<JobDetails> {
        todo!()
    }

    pub async fn get_job_status(&self, submission_id: &str) -> crate::Result<JobStatus> {
        todo!()
    }

    pub async fn get_job_logs(&self, submission_id: &str) -> crate::Result<JobLogsResponse> {
        todo!()
    }

    pub async fn stop_job(&self, submission_id: &str) -> crate::Result<JobStopResponse> {
        todo!()
    }

    pub async fn delete_job(&self, submission_id: &str) -> crate::Result<JobDeleteResponse> {
        todo!()
    }
}
