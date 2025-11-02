use std::collections::HashMap;

use crate::schemas::env::RuntimeEnv;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum JobType {
    SUBMISSION,
    DRIVER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Deserialize, serde::Serialize)]
pub enum JobStatus {
    PENDING,
    RUNNING,
    STOPPED,
    SUCCEEDED,
    FAILED,
}

impl JobStatus {
    pub fn is_terminal(&self) -> bool {
        matches!(
            self,
            JobStatus::SUCCEEDED | JobStatus::FAILED | JobStatus::STOPPED
        )
    }
}

#[derive(Debug, Default, Clone, serde::Deserialize, serde::Serialize)]
pub struct JobSubmitRequest {
    pub entrypoint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submission_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_env: Option<RuntimeEnv>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint_num_cpus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint_num_gpus: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint_memory: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint_resources: Option<HashMap<String, f64>>,
}

impl JobSubmitRequest {
    /// Create a new JobSubmitRequest with the required entrypoint.
    pub fn new(entrypoint: impl Into<String>) -> Self {
        Self {
            entrypoint: entrypoint.into(),
            ..Default::default()
        }
    }

    /// Set the submission ID for the job.
    pub fn with_submission_id(mut self, submission_id: impl Into<String>) -> Self {
        self.submission_id = Some(submission_id.into());
        self
    }

    /// Set the runtime environment for the job.
    pub fn with_runtime_env(mut self, runtime_env: RuntimeEnv) -> Self {
        self.runtime_env = Some(runtime_env);
        self
    }

    /// Set the metadata for the job.
    pub fn with_metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.metadata = Some(metadata);
        self
    }

    /// Add a single metadata item for the job.
    pub fn with_metadata_item(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        if self.metadata.is_none() {
            self.metadata = Some(HashMap::new());
        }
        self.metadata.as_mut().unwrap().insert(key.into(), value.into());
        self
    }

    /// Set the number of CPUs for the job's entrypoint.
    pub fn with_entrypoint_num_cpus(mut self, num_cpus: f64) -> Self {
        self.entrypoint_num_cpus = Some(num_cpus);
        self
    }

    /// Set the number of GPUs for the job's entrypoint.
    pub fn with_entrypoint_num_gpus(mut self, num_gpus: f64) -> Self {
        self.entrypoint_num_gpus = Some(num_gpus);
        self
    }

    /// Set the memory (in bytes) for the job's entrypoint.
    pub fn with_entrypoint_memory(mut self, memory: u64) -> Self {
        self.entrypoint_memory = Some(memory);
        self
    }

    /// Set custom resources for the job's entrypoint.
    pub fn with_entrypoint_resources(mut self, resources: HashMap<String, f64>) -> Self {
        self.entrypoint_resources = Some(resources);
        self
    }
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct JobDriverInfo {
    pub id: String,
    pub node_ip_address: String,
    pub pid: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct JobDetails {
    #[serde(alias = "type")]
    pub job_type: JobType,
    pub entrypoint: String,
    pub status: JobStatus,
    pub submission_id: Option<String>,
    pub driver_info: Option<JobDriverInfo>,
    pub message: Option<String>,
    pub error_type: Option<String>,
    pub start_time: Option<u64>,
    pub end_time: Option<u64>,
    pub metadata: Option<HashMap<String, String>>,
    pub runtime_env: Option<RuntimeEnv>,
    pub driver_agent_http_address: Option<String>,
    pub driver_node_id: Option<String>,
    pub driver_exit_code: Option<i32>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct JobVersionResponse {
    pub version: String,
    pub ray_version: String,
    pub ray_commit: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct JobSubmitResponse {
    pub submission_id: String,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct JobStopResponse {
    pub stopped: bool,
}

#[derive(Debug, Clone, Copy, serde::Deserialize, serde::Serialize)]
pub struct JobDeleteResponse {
    pub deleted: bool,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct JobLogsResponse {
    pub logs: String,
}

impl JobLogsResponse {
    pub fn lines(&self) -> impl Iterator<Item = &str> {
        self.logs.lines()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_type_serde() {
        let job_type = JobType::DRIVER;
        let serialized = serde_json::to_string(&job_type).unwrap();
        assert_eq!(serialized, r#""DRIVER""#);

        let deserialized: JobType = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, JobType::DRIVER);
    }

    #[test]
    fn test_job_status_serde() {
        let status = JobStatus::RUNNING;
        let serialized = serde_json::to_string(&status).unwrap();
        assert_eq!(serialized, r#""RUNNING""#);

        let deserialized: JobStatus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, JobStatus::RUNNING);
    }

    #[test]
    fn test_job_submit_request_builder() {
        let request = JobSubmitRequest::new("python script.py")
            .with_submission_id("submission_123")
            .with_entrypoint_num_cpus(4.0)
            .with_entrypoint_num_gpus(2.0)
            .with_entrypoint_memory(1024)
            .with_metadata_item("environment", "dev");

        assert!(request.submission_id.is_some());
        assert!(request.entrypoint_num_cpus.is_some());
        assert!(request.entrypoint_num_gpus.is_some());
        assert!(request.entrypoint_memory.is_some());
        assert!(request.metadata.is_some());
        assert!(request.metadata.unwrap().get("environment").unwrap() == "dev")
    }
}
